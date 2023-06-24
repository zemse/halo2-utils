mod example_circuit;

mod utils;
pub use utils::*;

pub mod layout_printer;
pub use layout_printer::LayoutPrinter;

pub mod real_prover;
pub use real_prover::RealProver;

use halo2_proofs::{arithmetic::Field, plonk::Circuit};
pub trait CircuitExt<F: Field>: Circuit<F> {
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
