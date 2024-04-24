use halo2_proofs_latest::halo2curves::bn256::Fr;
use halo2_utils::example_circuit::FactorisationCircuit;

fn main() {
    let circuit = FactorisationCircuit {
        a: Fr::from(2),
        b: Fr::from(3),
        _marker: std::marker::PhantomData,
    };
    println!("{:#?}", halo2_utils::infer_instance(&circuit, None));
}
