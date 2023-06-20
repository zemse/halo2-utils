use halo2_proofs::{
    halo2curves::bn256::{Bn256, Fr, G1Affine},
    plonk::{create_proof, keygen_pk, keygen_vk, Circuit, Error, ProvingKey, VerifyingKey},
    poly::{
        commitment::ParamsProver,
        kzg::{
            commitment::{KZGCommitmentScheme, ParamsKZG},
            multiopen::ProverSHPLONK,
        },
    },
    transcript::{Blake2bWrite, Challenge255, TranscriptWriterBuffer},
    SerdeFormat,
};
use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng, ChaChaRng};
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Clone)]
pub struct RealProver<'a, ConcreteCircuit: Circuit<Fr>> {
    name: &'a str,
    degree: u32,
    circuit: ConcreteCircuit,
    dir_path: PathBuf,
    serde_format: SerdeFormat,
    rng: ChaCha20Rng,
    general_params: Option<ParamsKZG<Bn256>>,
    verifier_params: Option<ParamsKZG<Bn256>>,
    circuit_proving_key: Option<ProvingKey<G1Affine>>,
    circuit_verifying_key: Option<VerifyingKey<G1Affine>>,
}

impl<'a, ConcreteCircuit: Circuit<Fr>> RealProver<'a, ConcreteCircuit> {
    pub fn init(name: &'a str, degree: u32, circuit: ConcreteCircuit) -> Self {
        Self {
            name,
            degree,
            circuit,
            dir_path: PathBuf::from_str("./out").unwrap(),
            serde_format: SerdeFormat::RawBytes,
            rng: ChaChaRng::seed_from_u64(2),
            general_params: None,
            verifier_params: None,
            circuit_proving_key: None,
            circuit_verifying_key: None,
        }
    }

    pub fn load(&mut self) -> Result<&Self, Error> {
        self.set_general_params(None)?;
        self.set_verifier_params(None)?;
        self.set_circuit_params(None, None)?;
        Ok(self)
    }

    pub fn run(mut self, instance: Vec<Vec<Fr>>, write_to_file: bool) -> Result<Vec<u8>, Error> {
        self.load()?;
        let instance_refs: Vec<&[Fr]> = instance.iter().map(|v| &v[..]).collect();
        let mut transcript = Blake2bWrite::<_, G1Affine, Challenge255<_>>::init(vec![]);
        create_proof::<
            KZGCommitmentScheme<Bn256>,
            ProverSHPLONK<'_, Bn256>,
            Challenge255<G1Affine>,
            ChaChaRng,
            Blake2bWrite<Vec<u8>, G1Affine, Challenge255<G1Affine>>,
            _,
        >(
            &self.general_params.as_mut().unwrap(),
            &self.circuit_proving_key.as_mut().unwrap(),
            &[self.circuit],
            &[&instance_refs],
            self.rng.to_owned(),
            &mut transcript,
        )
        .unwrap();

        let proof = transcript.finalize();
        if write_to_file {
            let proof_path = self
                .dir_path
                .join(Path::new(&format!("{}_proof", self.name)));

            let mut file = File::create(proof_path)?;
            file.write(proof.as_slice())?;
        }
        Ok(proof)
    }

    fn set_general_params(
        &mut self,
        params_override: Option<ParamsKZG<Bn256>>,
    ) -> Result<(), Error> {
        if self.general_params.is_some() {
            return Ok(());
        }

        if params_override.is_some() {
            self.general_params = params_override;
            return Ok(());
        }

        self.ensure_dir_exists();

        let path = self
            .dir_path
            .join(Path::new(&format!("kzg_general_params_{}", self.degree)));
        match File::open(path.clone()) {
            Ok(mut file) => {
                println!("reading {}", path.display());
                self.general_params = Some(ParamsKZG::<Bn256>::read_custom(
                    &mut file,
                    self.serde_format,
                )?);
            }
            Err(_) => {
                println!("setting up general params");
                let general_params = ParamsKZG::<Bn256>::setup(self.degree, self.rng.clone());
                println!("writing {}", path.display());
                let mut file = File::create(path)?;
                general_params.write_custom(&mut file, self.serde_format)?;
                self.general_params = Some(general_params);
            }
        };
        Ok(())
    }

    fn set_verifier_params(
        &mut self,
        params_override: Option<ParamsKZG<Bn256>>,
    ) -> Result<(), Error> {
        if self.verifier_params.is_some() {
            return Ok(());
        }

        if params_override.is_some() {
            self.verifier_params = params_override;
            return Ok(());
        }

        self.ensure_dir_exists();

        let path = self
            .dir_path
            .join(Path::new(&format!("kzg_verifier_params_{}", self.degree)));
        match File::open(path.clone()) {
            Ok(mut file) => {
                println!("reading {}", path.display());
                self.verifier_params = Some(ParamsKZG::<Bn256>::read_custom(
                    &mut file,
                    self.serde_format,
                )?);
            }
            Err(_) => {
                println!("setting up verifier params");
                let general_params = self.general_params.clone().unwrap();
                let verifier_params = general_params.verifier_params().to_owned();
                println!("writing {}", path.display());
                let mut file = File::create(path)?;
                verifier_params.write_custom(&mut file, self.serde_format)?;
                self.verifier_params = Some(verifier_params);
            }
        };
        Ok(())
    }

    pub fn set_circuit_params(
        &mut self,
        circuit_proving_key_override: Option<ProvingKey<G1Affine>>,
        circuit_verifying_key_override: Option<VerifyingKey<G1Affine>>,
    ) -> Result<(), Error> {
        if self.circuit_proving_key.is_some() && self.circuit_verifying_key.is_some() {
            return Ok(());
        }

        if circuit_proving_key_override.is_some() && circuit_verifying_key_override.is_some() {
            self.circuit_proving_key = circuit_proving_key_override;
            self.circuit_verifying_key = circuit_verifying_key_override;
            return Ok(());
        }

        let verifying_key_path = self.dir_path.join(Path::new(&format!(
            "{}_verifying_key_{}",
            self.name, self.degree
        )));
        match File::open(verifying_key_path.clone()) {
            Ok(mut file) => {
                println!("reading {}", verifying_key_path.display());
                self.circuit_verifying_key = Some(
                    VerifyingKey::<G1Affine>::read::<File, ConcreteCircuit>(
                        &mut file,
                        self.serde_format,
                    )
                    .unwrap(),
                );
            }
            Err(_) => {
                println!("setting up verifying key");
                let vk = keygen_vk(self.general_params.as_mut().unwrap(), &self.circuit)
                    .expect("keygen_vk should not fail");
                println!("writing {}", verifying_key_path.display());
                let mut file = File::create(verifying_key_path)?;
                vk.write(&mut file, self.serde_format)?;
                self.circuit_verifying_key = Some(vk);
            }
        };

        self.ensure_dir_exists();

        let proving_key_path = self.dir_path.join(Path::new(&format!(
            "{}_proving_key_{}",
            self.name, self.degree
        )));
        match File::open(proving_key_path.clone()) {
            Ok(mut file) => {
                println!("reading {}", proving_key_path.display());
                self.circuit_proving_key = Some(
                    ProvingKey::<G1Affine>::read::<File, ConcreteCircuit>(
                        &mut file,
                        self.serde_format,
                    )
                    .unwrap(),
                );
            }
            Err(_) => {
                println!("setting up proving key");
                let pk = keygen_pk(
                    self.general_params.as_mut().unwrap(),
                    self.circuit_verifying_key.clone().unwrap(),
                    &self.circuit,
                )
                .expect("keygen_pk should not fail");
                println!("writing {}", proving_key_path.display());
                let mut file = File::create(proving_key_path)?;
                pk.write(&mut file, self.serde_format)?;
                self.circuit_proving_key = Some(pk);
            }
        };
        Ok(())
    }

    fn ensure_dir_exists(&self) {
        create_dir_all(self.dir_path.clone()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use halo2_proofs::halo2curves::bn256::Fr;

    use super::*;
    use crate::example_circuit::MyCircuit;

    #[test]
    fn it_works() {
        let circuit = MyCircuit::<Fr>::default();
        let prover = RealProver::init("MyCircuit", 4, circuit);
        prover.run(vec![vec![Fr::from(0)]], true).unwrap();
    }
}
