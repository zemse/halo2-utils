use halo2_proofs::dev::MockProver;

use crate::{CircuitExt, FieldExt};

pub fn print<F: FieldExt, C: CircuitExt<F>>(k: u32, circuit: &C) {
    let prover: MockProver<F> = MockProver::run(k, circuit, circuit.instances()).unwrap();

    let cs = prover.cs();

    println!("advice columns: {:?}", prover.advice().len());
    println!("fixed columns: {:?}", prover.fixed().len());
    println!("instance columns: {:?}", prover.instance().len());
    println!("selectors columns: {:?}", prover.selectors().len());
    println!("gates: {:?}", cs.gates().len());
    println!("lookups: {:?}", cs.lookups().len());
}
