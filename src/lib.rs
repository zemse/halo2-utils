#![feature(int_log)]

pub mod example_circuit;

pub mod error;

mod utils;

pub use utils::*;

#[cfg(feature = "latest-halo2")]
pub mod assignments_printer;
#[cfg(feature = "latest-halo2")]
pub use assignments_printer as assignments;

pub mod info_printer;
pub use info_printer as info;

mod layout_printer;
pub use layout_printer::LayoutPrinter;

#[cfg(any(feature = "v030-halo2", feature = "latest-halo2"))]
pub mod real_prover;
#[cfg(any(feature = "v030-halo2", feature = "latest-halo2"))]
pub use real_prover::RealProver;

// #[cfg(feature = "latest-halo2")]
mod estimate_k;
// #[cfg(feature = "latest-halo2")]
pub use estimate_k::estimate_k;

mod infer_instance;
pub use infer_instance::infer_instance;

use halo2_proofs::plonk::Circuit;

pub mod field;
pub use field::{FieldExt, RawField};

// pub mod compare;

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
// pub use ethers;
// pub use halo2_gadgets;

#[cfg(feature = "latest-halo2")]
pub use halo2_proofs_latest as halo2_proofs;

#[cfg(feature = "v030-halo2")]
pub use halo2_proofs_v030 as halo2_proofs;

#[cfg(feature = "v2022_08_19-halo2")]
pub use halo2_proofs_v2022_08_19 as halo2_proofs;

pub use plotters;
pub use rand_chacha;
#[cfg(feature = "evm-verifier")]
pub use snark_verifier;
#[cfg(feature = "evm-verifier")]
pub use snark_verifier_sdk;
