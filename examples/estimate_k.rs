#[cfg(any(feature = "latest-halo2", feature = "v030-halo2"))]
use halo2_utils::{example_circuit::FactorisationCircuit, halo2_proofs::halo2curves::bn256::Fr};

fn main() {
    #[cfg(any(feature = "latest-halo2", feature = "v030-halo2"))]
    let circuit = FactorisationCircuit {
        a: Fr::from(2),
        b: Fr::from(3),
        _marker: std::marker::PhantomData,
    };
    #[cfg(any(feature = "latest-halo2", feature = "v030-halo2"))]
    println!("{:?}", halo2_utils::estimate_k(&circuit));

    #[cfg(not(any(feature = "latest-halo2", feature = "v030-halo2")))]
    panic!("This example is only supported under the 'latest-halo2' or 'v030-halo2' feature")
}
