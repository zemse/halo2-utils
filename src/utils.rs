use halo2_proofs::{
    arithmetic::Field,
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
        InstanceValue::Padding => F::ZERO,
    }
}

pub fn parse_cell_value<F: RawField>(value: CellValue<F>) -> F {
    match value {
        CellValue::Unassigned => F::ZERO,
        CellValue::Assigned(f) => f,
        CellValue::Poison(v) => F::from(v as u64),
    }
}
