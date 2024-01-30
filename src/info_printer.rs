use halo2_proofs::dev::MockProver;

use crate::{estimate_k, CircuitExt, FieldExt};

/// Prints the info for the circuit.
pub fn print<F: FieldExt, C: CircuitExt<F>>(circuit: &C, k: Option<u32>) {
    let k = k.unwrap_or_else(|| estimate_k(circuit));
    let prover: MockProver<F> = MockProver::run(k, circuit, circuit.instances()).unwrap();

    let cs = prover.cs();

    println!("advice columns: {:?}", prover.advice().len());
    println!("fixed columns: {:?}", prover.fixed().len());
    println!("instance columns: {:?}", prover.instance().len());
    println!("selectors columns: {:?}", prover.selectors().len());
    println!("gates: {:?}", cs.gates().len());
    println!("lookups: {:?}", cs.lookups().len());
}
