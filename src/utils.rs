use halo2_proofs::{
    arithmetic::Field,
    plonk::{Circuit, ConstraintSystem},
};

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
