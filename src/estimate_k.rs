use crate::halo2_proofs::{dev::MockProver, plonk::Circuit};

use crate::RawField;

/// Performs synthesis on a huge plonkish table and then sees how many rows were actually used.
pub fn estimate_k<F: RawField, C: Circuit<F>>(circuit: &C) -> u32 {
    let rows = estimate_rows::<F, C>(circuit);
    rows.ilog2() + 1
}

pub fn estimate_rows<F: RawField, C: Circuit<F>>(circuit: &C) -> u32 {
    let num_instance = crate::infer_instance::get_number_of_instance_columns::<F, C>(
        #[cfg(feature = "circuit-params")]
        circuit,
    );
    let prover = MockProver::run(26, circuit, vec![vec![]; num_instance]).unwrap();

    let mut last_row = 0;
    for region in prover.regions() {
        if let Some((start, end)) = region.rows() {
            assert!(end >= start);
            if end > last_row {
                last_row = end;
            }
        }
    }

    (last_row + prover.cs().blinding_factors() + 1) as u32
}
