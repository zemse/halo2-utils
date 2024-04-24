use crate::halo2_proofs::{
    arithmetic::{Field, Group},
    dev::{CellValue, InstanceValue},
    plonk::{Circuit, ConstraintSystem},
};

use std::fmt::Debug;

use crate::RawField;

pub fn derive_k<F, ConcreteCircuit>() -> u32
where
    F: Field,
    ConcreteCircuit: Circuit<F>,
{
    let mut cs = ConstraintSystem::<F>::default();
    ConcreteCircuit::configure(&mut cs);
    let rows: u32 = cs.minimum_rows().try_into().unwrap();
    rows.ilog2() + 1
}

pub fn derive_circuit_name<ConcreteCircuit>(circuit: &ConcreteCircuit) -> String
where
    ConcreteCircuit: Debug,
{
    let mut circuit_format = format!("{:?}", circuit);
    if let Some(index) = circuit_format.find(' ') {
        circuit_format.truncate(index);
        circuit_format
    } else {
        panic!("no space found in '{}'", circuit_format);
    }
}

pub fn instance_value<F: Field>(val: &InstanceValue<F>) -> F {
    match val {
        InstanceValue::Assigned(v) => *v,
        #[cfg(any(feature = "v030-halo2", feature = "latest-halo2"))]
        InstanceValue::Padding => F::ZERO,
        #[cfg(not(any(feature = "v030-halo2", feature = "latest-halo2")))]
        InstanceValue::Padding => F::zero(),
    }
}

pub fn parse_cell_value<F: RawField + Group>(value: CellValue<F>) -> F {
    match value {
        #[cfg(any(feature = "v030-halo2", feature = "latest-halo2"))]
        CellValue::Unassigned => F::ZERO,
        #[cfg(not(any(feature = "v030-halo2", feature = "latest-halo2")))]
        CellValue::Unassigned => F::zero(),
        CellValue::Assigned(f) => f,
        CellValue::Poison(v) => F::from(v as u64),
        // CellValue::Rational(n, _) => n,
    }
}
