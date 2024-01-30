pub mod example_circuit;

pub mod error;

mod utils;

pub use utils::*;

pub mod assignments_printer;
pub use assignments_printer as assignments;
pub mod info_printer;
pub use info_printer as info;

mod layout_printer;
pub use layout_printer::LayoutPrinter;

pub mod real_prover;
pub use real_prover::RealProver;

mod estimate_k;
pub use estimate_k::estimate_k;

mod infer_instance;
pub use infer_instance::infer_instance;

use halo2_proofs::plonk::Circuit;

pub mod field;
pub use field::{FieldExt, RawField};

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

pub mod zkevm;
pub use zkevm::{Expr, Scalar};

// export dependencies
pub use ethers;
// pub use halo2_gadgets;
pub use halo2_proofs;
pub use plotters;
pub use rand_chacha;
#[cfg(feature = "evm-verifier")]
pub use snark_verifier;
#[cfg(feature = "evm-verifier")]
pub use snark_verifier_sdk;
