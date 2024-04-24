use halo2_utils::{example_circuit::FactorisationCircuit, halo2_proofs::halo2curves::bn256::Fr};

fn main() {
    halo2_utils::info::print::<Fr, FactorisationCircuit<Fr>>();
}

// output
//
// advice columns: 2
// fixed columns: 1
// instance columns: 1
// selectors columns: 1
// gates: 1
// lookups: 0
