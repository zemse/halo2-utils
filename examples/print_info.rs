use halo2_utils::{example_circuit::FactorisationCircuit, halo2_proofs::halo2curves::bn256::Fr};

fn main() {
    let circuit = FactorisationCircuit {
        a: Fr::from(2),
        b: Fr::from(3),
        _marker: std::marker::PhantomData,
    };

    halo2_utils::info::print(&circuit);
}

// output
//
// advice columns: 2
// fixed columns: 1
// instance columns: 1
// selectors columns: 1
// gates: 1
// lookups: 0
