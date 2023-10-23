pub mod example_circuit;

pub mod error;

mod utils;

pub use utils::*;

pub mod assignments_printer;
pub mod info_printer;

pub mod layout_printer;
pub use layout_printer::LayoutPrinter;

pub mod real_prover;
pub use real_prover::RealProver;

use halo2_proofs::{
    halo2curves::{bn256::Fr, ff::FromUniformBytes, pasta::Fp},
    plonk::Circuit,
};
pub trait CircuitExt<F: FieldExt>: Circuit<F> {
    /// Annotations for advice, fixed, instance and selector columns.
    fn annotations(&self) -> (Vec<&str>, Vec<&str>, Vec<&str>, Vec<&str>);

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

pub trait FieldExt:
    halo2_proofs::arithmetic::Field + From<u64> + FromUniformBytes<64> + Ord
{
}

impl FieldExt for Fr {}
impl FieldExt for Fp {}

pub mod zkevm;
pub use zkevm::{Expr, Scalar};

// export dependencies
pub use ethers;
pub use halo2_gadgets;
pub use halo2_proofs;
pub use plotters;
pub use rand_chacha;
#[cfg(feature = "evm-verifier")]
pub use snark_verifier;
#[cfg(feature = "evm-verifier")]
pub use snark_verifier_sdk;
