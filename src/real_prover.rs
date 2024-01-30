#[allow(unused_imports)]
use crate::{derive_circuit_name, error::Error, CircuitExt, FieldExt};
use halo2_proofs::{
    halo2curves::bn256::{Bn256, Fr, G1Affine},
    plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, Circuit, ProvingKey, VerifyingKey},
    poly::{
        commitment::ParamsProver,
        kzg::{
            commitment::{KZGCommitmentScheme, ParamsKZG},
            multiopen::{ProverSHPLONK, VerifierSHPLONK},
            strategy::SingleStrategy,
        },
    },
    transcript::{
        Blake2bRead, Blake2bWrite, Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer,
    },
    SerdeFormat,
};
use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng, ChaChaRng};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
    time::Instant,
};

#[cfg(feature = "evm-verifier")]
use halo2_proofs::halo2curves::bn256::Fq;
#[cfg(feature = "evm-verifier")]
use snark_verifier::{
    loader::evm::{encode_calldata, EvmLoader},
    pcs::kzg::{Gwc19, KzgAs, KzgDecidingKey},
    system::halo2::{compile, transcript::evm::EvmTranscript, Config},
    verifier::{self, SnarkVerifier},
};
#[cfg(feature = "evm-verifier")]
use std::rc::Rc;

#[cfg(feature = "evm-verifier")]
type PlonkVerifier = verifier::plonk::PlonkVerifier<KzgAs<Bn256, Gwc19>>;

#[derive(Clone)]
pub struct RealProver<ConcreteCircuit: Circuit<Fr> + CircuitExt<Fr> + Clone + Debug> {
    circuit: ConcreteCircuit,
    degree: u32,
    dir_path: PathBuf,
    serde_format: SerdeFormat,
    rng: ChaCha20Rng,
    pub general_params: Option<ParamsKZG<Bn256>>,
    pub verifier_params: Option<ParamsKZG<Bn256>>,
    pub circuit_proving_key: Option<ProvingKey<G1Affine>>,
    pub circuit_verifying_key: Option<VerifyingKey<G1Affine>>,
}

impl<ConcreteCircuit: Circuit<Fr> + CircuitExt<Fr> + Clone + Debug> RealProver<ConcreteCircuit> {
    pub fn from(degree: u32, circuit: ConcreteCircuit) -> Self {
        Self {
            circuit,
            degree, //: derive_k::<Fr, ConcreteCircuit>(),
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

    pub fn run(&mut self) -> Result<Proof, Error> {
        self.load()?;
        let instances = self.circuit.instances();
        let instances_refs_intermediate = instances.iter().map(|v| &v[..]).collect::<Vec<&[Fr]>>();
        let mut transcript = Blake2bWrite::<_, G1Affine, Challenge255<_>>::init(vec![]);
        let now = Instant::now();
        create_proof::<
            KZGCommitmentScheme<Bn256>,
            ProverSHPLONK<'_, Bn256>,
            Challenge255<G1Affine>,
            ChaChaRng,
            Blake2bWrite<Vec<u8>, G1Affine, Challenge255<G1Affine>>,
            _,
        >(
            self.general_params.as_mut().unwrap(),
            self.circuit_proving_key.as_mut().unwrap(),
            &[self.circuit.clone()],
            &[&instances_refs_intermediate],
            self.rng.to_owned(),
            &mut transcript,
        )
        .unwrap();
        let elapsed = now.elapsed();
        println!("Proof generation took {:?}", elapsed);

        let proof = transcript.finalize();
        Ok(Proof::from(
            self.degree,
            proof,
            instances,
            derive_circuit_name::<ConcreteCircuit>(&self.circuit),
        ))
    }

    pub fn verifier(&self) -> RealVerifier {
        RealVerifier {
            circuit_name: derive_circuit_name(&self.circuit),
            dir_path: self.dir_path.clone(),
            num_instance: self.circuit.num_instance(),
            general_params: self
                .general_params
                .clone()
                .ok_or("params not available, please execute prover.load() first")
                .unwrap(),
            verifier_params: self.verifier_params.clone().unwrap(),
            circuit_verifying_key: self.circuit_verifying_key.clone().unwrap(),
        }
    }

    pub fn degree(mut self, k: u32) -> Self {
        self.degree = k;
        self
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
                self.general_params = Some(ParamsKZG::<Bn256>::read_custom(
                    &mut file,
                    self.serde_format,
                )?);
            }
            Err(_) => {
                let general_params = ParamsKZG::<Bn256>::setup(self.degree, self.rng.clone());
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
                self.verifier_params = Some(ParamsKZG::<Bn256>::read_custom(
                    &mut file,
                    self.serde_format,
                )?);
            }
            Err(_) => {
                let general_params = self.general_params.clone().unwrap();
                let verifier_params = general_params.verifier_params().to_owned();
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
            derive_circuit_name(&self.circuit),
            self.degree
        )));
        match File::open(verifying_key_path.clone()) {
            Ok(mut file) => {
                self.circuit_verifying_key = Some(
                    VerifyingKey::<G1Affine>::read::<File, ConcreteCircuit>(
                        &mut file,
                        self.serde_format,
                    )
                    .unwrap(),
                );
            }
            Err(_) => {
                let vk = keygen_vk(self.general_params.as_mut().unwrap(), &self.circuit)
                    .expect("keygen_vk should not fail");
                let mut file = File::create(verifying_key_path)?;
                vk.write(&mut file, self.serde_format)?;
                self.circuit_verifying_key = Some(vk);
            }
        };

        self.ensure_dir_exists();

        let proving_key_path = self.dir_path.join(Path::new(&format!(
            "{}_proving_key_{}",
            derive_circuit_name(&self.circuit),
            self.degree
        )));
        match File::open(proving_key_path.clone()) {
            Ok(mut file) => {
                self.circuit_proving_key = Some(
                    ProvingKey::<G1Affine>::read::<File, ConcreteCircuit>(
                        &mut file,
                        self.serde_format,
                    )
                    .unwrap(),
                );
            }
            Err(_) => {
                let pk = keygen_pk(
                    self.general_params.as_mut().unwrap(),
                    self.circuit_verifying_key.clone().unwrap(),
                    &self.circuit,
                )
                .expect("keygen_pk should not fail");
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proof {
    degree: u32,
    data: Vec<u8>,
    instances: Vec<Vec<Fr>>,
    circuit_name: String,
}

impl Proof {
    pub fn from(
        degree: u32,
        proof: Vec<u8>,
        instances: Vec<Vec<Fr>>,
        circuit_name: String,
    ) -> Self {
        Self {
            degree,
            data: proof,
            instances,
            circuit_name,
        }
    }

    pub fn read_from_file(path: &PathBuf) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn degree(&self) -> u32 {
        self.degree
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn instances(&self) -> &Vec<Vec<Fr>> {
        &self.instances
    }

    pub fn circuit_name(&self) -> &String {
        &self.circuit_name
    }

    pub fn num_instances(&self) -> Vec<usize> {
        self.instances.iter().map(|column| column.len()).collect()
    }

    pub fn unpack(&self) -> (u32, Vec<u8>, Vec<Vec<Fr>>, String) {
        (
            self.degree,
            self.data.clone(),
            self.instances.clone(),
            self.circuit_name.clone(),
        )
    }

    pub fn write_to_file(&self, path: &PathBuf) -> Result<(), Error> {
        let mut file = File::create(path)?;
        file.write_all(serde_json::to_string(self)?.as_bytes())
            .unwrap();
        Ok(())
    }

    #[cfg(feature = "evm-verifier")]
    pub fn encode_calldata(&self) -> Vec<u8> {
        encode_calldata(self.instances(), self.data())
    }
}

pub struct RealVerifier {
    pub circuit_name: String,
    pub dir_path: PathBuf,
    pub num_instance: Vec<usize>,
    pub general_params: ParamsKZG<Bn256>,
    pub verifier_params: ParamsKZG<Bn256>,
    pub circuit_verifying_key: VerifyingKey<G1Affine>,
}

impl RealVerifier {
    pub fn run(&self, proof: Proof) -> Result<(), Error> {
        let strategy = SingleStrategy::new(&self.general_params);
        let instance_refs_intermediate = proof
            .instances()
            .iter()
            .map(|v| &v[..])
            .collect::<Vec<&[Fr]>>();
        let mut verifier_transcript =
            Blake2bRead::<_, G1Affine, Challenge255<_>>::init(&proof.data()[..]);

        verify_proof::<
            KZGCommitmentScheme<Bn256>,
            VerifierSHPLONK<'_, Bn256>,
            Challenge255<G1Affine>,
            Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
            SingleStrategy<'_, Bn256>,
        >(
            &self.verifier_params,
            &self.circuit_verifying_key,
            strategy,
            &[&instance_refs_intermediate],
            &mut verifier_transcript,
        )?;
        Ok(())
    }

    #[cfg(feature = "evm-verifier")]
    pub fn generate_yul(&self, path: Option<&PathBuf>) -> Result<String, Error> {
        let protocol = compile(
            &self.verifier_params,
            &self.circuit_verifying_key,
            Config::kzg().with_num_instance(self.num_instance.clone()),
        );
        let vk: KzgDecidingKey<Bn256> = (
            self.verifier_params.get_g()[0],
            self.verifier_params.g2(),
            self.verifier_params.s_g2(),
        )
            .into();

        let loader = EvmLoader::new::<Fq, Fr>();
        let protocol = protocol.loaded(&loader);
        let mut transcript = EvmTranscript::<_, Rc<EvmLoader>, _, _>::new(&loader);

        let instances = transcript.load_instances(self.num_instance.clone());
        let proof = PlonkVerifier::read_proof(&vk, &protocol, &instances, &mut transcript).unwrap();
        PlonkVerifier::verify(&vk, &protocol, &instances, &proof).unwrap();

        let source = loader.solidity_code();
        if let Some(path) = path {
            let mut file = File::create(path)?;
            file.write_all(source.as_bytes())?;
        }
        Ok(source)
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use halo2_proofs::halo2curves::bn256::Fr;

    use super::*;
    use crate::example_circuit::FactorisationCircuit;

    #[test]
    fn it_works() {
        let mut prover = RealProver::from(
            4,
            FactorisationCircuit {
                a: Fr::from(3),
                b: Fr::from(7),
                _marker: PhantomData,
            },
        );
        let proof = prover.run().unwrap();

        let verifier = prover.verifier();
        let result = verifier.run(proof);
        assert!(result.is_ok());
    }
}
