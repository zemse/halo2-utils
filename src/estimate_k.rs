use halo2_proofs::{dev::MockProver, halo2curves::ff};

use crate::{CircuitExt, FieldExt};

/// Performs synthesis on a huge plonkish table and then sees how many rows were actually used.
pub fn estimate_k<F: FieldExt + ff::PrimeField, C: CircuitExt<F>>(circuit: &C) -> u32 {
    let prover = MockProver::run(26, circuit, circuit.instances()).unwrap();
    let rows = prover.last_row() + prover.cs().blinding_factors() + 1;
    rows.ilog2() + 1
}
