use halo2_proofs::plonk::{Circuit, ConstraintSystem};

use crate::RawField;

/// Prints the info for the circuit.
pub fn print<F: RawField, C: Circuit<F>>(circuit: &C) {
    let mut cs = ConstraintSystem::default();
    #[cfg(feature = "circuit-params")]
    C::configure_with_params(&mut cs, circuit.params());
    #[cfg(not(feature = "circuit-params"))]
    C::configure(&mut cs);

    println!("advice columns: {:?}", cs.num_advice_columns());
    println!("fixed columns: {:?}", cs.num_fixed_columns());
    println!("instance columns: {:?}", cs.num_instance_columns());
    println!("selectors columns: {:?}", cs.num_selectors());
    println!("gates: {:?}", cs.gates().len());
    println!("lookups: {:?}", cs.lookups().len());
}
