mod example_circuit;

mod utils;
pub use utils::*;

pub mod layout_printer;
pub use layout_printer::LayoutPrinter;

pub mod real_prover;
pub use real_prover::RealProver;

use halo2_proofs::{
    halo2curves::{bn256::Fr, pasta::Fp},
    plonk::Circuit,
};
pub trait CircuitExt<F: FieldExt>: Circuit<F> {
    /// Return the instances of the circuit.
    /// This may depend on extra circuit parameters but NOT on private witnesses.
    fn instances(&self) -> Vec<Vec<F>>;

    fn num_instance(&self) -> Vec<usize> {
        self.instances()
            .iter()
            .map(|inst| inst.len())
            .collect::<Vec<usize>>()
    }
}

pub trait FieldExt: halo2_proofs::arithmetic::Field + From<u64> {}

impl FieldExt for Fr {}
impl FieldExt for Fp {}

pub mod zkevm;
pub use zkevm::{Expr, Scalar};

// export dependencies
pub use halo2_proofs;
pub use plotters;
pub use rand_chacha;
pub use snark_verifier;
pub use snark_verifier_sdk;
