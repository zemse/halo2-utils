use halo2_proofs::{
    dev::MockProver,
    plonk::{Any, Circuit, ConstraintSystem},
};

use crate::{estimate_k, parse_cell_value, RawField};

pub fn get_number_of_instance_columns<F: RawField, C: Circuit<F>>() -> usize {
    let mut cs = ConstraintSystem::<F>::default();
    C::configure(&mut cs);
    cs.num_instance_columns()
}

/// Finds the instances for the circuit using copy constraints.
pub fn infer_instance<F: RawField, C: Circuit<F>>(circuit: &C, k: Option<u32>) -> Vec<Vec<F>> {
    let k = k.unwrap_or_else(|| estimate_k(circuit));
    let num_instance = get_number_of_instance_columns::<F, C>();
    let instance = vec![vec![]; num_instance];
    let prover: MockProver<F> = MockProver::run(k, circuit, instance).unwrap();
    let copy_constraints = prover.permutation().copy_constraints();

    let mut instance: Vec<Vec<F>> = vec![vec![]; num_instance];
    for (left_column, left_row, right_column, right_row) in copy_constraints {
        let is_left_instance = matches!(left_column.column_type(), Any::Instance);
        let is_right_instance = matches!(right_column.column_type(), Any::Instance);

        assert!(
            !is_left_instance || !is_right_instance,
            "both should not be instance"
        );

        if is_left_instance || is_right_instance {
            let (instance_column, instance_row, other_column, other_row) = if is_left_instance {
                (left_column, left_row, right_column, right_row)
            } else {
                (right_column, right_row, left_column, left_row)
            };

            let other_value = match other_column.column_type() {
                Any::Advice(_) => prover.advice()[other_column.index()][*other_row],
                Any::Fixed => prover.fixed()[other_column.index()][*other_row],
                Any::Instance => unreachable!(),
            };

            let col = &mut instance[instance_column.index()];
            while col.len() < *instance_row {
                col.push(F::ZERO);
            }

            col.push(parse_cell_value(other_value))
        }
    }

    instance
}
