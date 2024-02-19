use std::ops::IndexMut;

use ethers::types::{BigEndianHash, H256, U256};
use halo2_proofs::{
    dev::{CellValue, InstanceValue, MockProver},
    plonk::Circuit,
};

use crate::{estimate_k, infer_instance::get_number_of_instance_columns, RawField};

#[derive(Debug, PartialEq)]
pub enum Column {
    Advice(usize),
    Fixed(usize),
    Instance(usize),
    Selector(usize),
}
/// Prints all the columns in the table.
pub fn compare_all<F: RawField, C1: Circuit<F>, C2: Circuit<F>>(
    circuit1: &C1,
    circuit2: &C2,
    k: Option<u32>,
) where
    F::Repr: Sized + IndexMut<usize>,
{
    let k1 = k.unwrap_or_else(|| estimate_k(circuit1));
    let k2 = k.unwrap_or_else(|| estimate_k(circuit2));
    let k = std::cmp::max(k1, k2);

    let num_instance1 = get_number_of_instance_columns(circuit1);
    let prover1 = MockProver::run(k, circuit1, vec![vec![]; num_instance1]).unwrap();

    let num_instance2 = get_number_of_instance_columns(circuit2);
    let prover2 = MockProver::run(k, circuit2, vec![vec![]; num_instance2]).unwrap();

    // let range = prover1.usable_rows();

    let advice1 = prover1.advice();
    let fixed1 = prover1.fixed();
    // let instance1 = prover1.instance();
    let (advice_annotations1, fixed_annotations1, _instance_annotations1) =
        get_annotations(&prover1);

    let advice2 = prover2.advice();
    let fixed2 = prover2.fixed();
    // let instance2 = prover2.instance();
    // let (advice_annotations2, fixed_annotations2, instance_annotations2) =
    //     get_annotations(&prover2);

    let mut mismatches = 0;
    for (col_i, a1) in advice1.iter().enumerate() {
        println!("checking advice {} {:?}", col_i, advice_annotations1[col_i]);
        let mut mismatches_local = 0;
        let mut mismatch_arr = vec![];

        if advice2.len() <= col_i {
            println!(
                "advice2 does not have column {} - {:?}",
                col_i, advice_annotations1[col_i]
            );
            continue;
        }

        for (row_i, cell_1) in a1.iter().enumerate() {
            if unwrap_cell_value(*cell_1) != unwrap_cell_value(advice2[col_i][row_i]) {
                mismatches += 1;
                mismatches_local += 1;
                mismatch_arr.push(format!(
                    "row {} - left {:?} != right {:?}",
                    row_i,
                    unwrap_cell_value(*cell_1),
                    unwrap_cell_value(advice2[col_i][row_i])
                ));
            }
        }
        if mismatches_local > 0 {
            println!(
                "found mismatch in col {:?} - # {}",
                advice_annotations1[col_i], mismatches_local
            );
        }
        if mismatches_local < 10 {
            for m in mismatch_arr {
                println!("  {}", m);
            }
        }
    }
    println!("total advice mismatches: {}", mismatches);

    for (col_i, f1) in fixed1.iter().enumerate() {
        println!("checking fixed {} {:?}", col_i, fixed_annotations1[col_i]);
        let mut mismatches_local = 0;
        let mut mismatch_arr = vec![];

        if fixed2.len() <= col_i {
            println!(
                "fixed2 does not have column {} - {:?}",
                col_i, fixed_annotations1[col_i]
            );
            continue;
        }

        for (row_i, cell_1) in f1.iter().enumerate() {
            if unwrap_cell_value(*cell_1) != unwrap_cell_value(fixed2[col_i][row_i]) {
                mismatches += 1;
                mismatches_local += 1;
                mismatch_arr.push(format!(
                    "row {} - left {:?} != right {:?}",
                    row_i,
                    unwrap_cell_value(*cell_1),
                    unwrap_cell_value(fixed2[col_i][row_i])
                ));
            }
        }
        if mismatches_local > 0 {
            println!(
                "found mismatch in col {:?} - # {}",
                fixed_annotations1[col_i], mismatches_local
            );
        }
        if mismatches_local < 225 {
            for m in mismatch_arr {
                println!("  {}", m);
            }
        }
    }
    println!("total mismatches: {}", mismatches);

    // for row_id in range.start..=range.end {
    //     let mut row_data = vec![];
    //     for col in advice1 {
    //         row_data.push(format_cell_value(col[row_id]));
    //     }
    //     for col in fixed1 {
    //         row_data.push(format_cell_value(col[row_id]));
    //     }
    //     for col in instance1 {
    //         row_data.push(format_value(instance_value(&col[row_id])));
    //     }
    // }
}

pub fn format_cell_value<F: RawField>(value: CellValue<F>) -> String {
    match value {
        CellValue::Unassigned => "Unassigned".to_string(),
        CellValue::Assigned(f) => format_value(f),
        CellValue::Poison(v) => format!("Poisoned({})", v),
    }
}

pub fn unwrap_cell_value<F: RawField>(value: CellValue<F>) -> F {
    match value {
        CellValue::Unassigned => F::ZERO,
        CellValue::Assigned(f) => f,
        CellValue::Poison(v) => F::from(v as u64),
    }
}

pub fn unwrap_instance_value<F: RawField>(value: &InstanceValue<F>) -> F {
    match value {
        InstanceValue::Assigned(f) => *f,
        InstanceValue::Padding => F::ZERO,
    }
}

// fn format_instance_value<F: FieldExt + ff::PrimeField>(value: InstanceValue<F>) -> String {
//     match value {
//         InstanceValue::Assigned(f) => format_value(f),
//         InstanceValue::Padding => "Padding".to_string(),
//     }
// }

fn format_value<F: RawField>(f: F) -> String {
    let v = f.to_repr();
    let v = v.as_ref();
    let v = U256::from_little_endian(v);
    if v > U256::from(u64::MAX) {
        format!("{:?}", H256::from_uint(&v))
    } else {
        format!("{:x}", v)
    }
}

#[allow(clippy::type_complexity)]
fn get_annotations<F: RawField>(
    prover: &MockProver<F>,
) -> (Vec<Option<&str>>, Vec<Option<&str>>, Vec<Option<&str>>)
where
    F::Repr: Sized + IndexMut<usize>,
{
    let advice = prover.advice();
    let fixed = prover.fixed();
    let instance = prover.instance();

    let mut advice_annotations: Vec<Option<&str>> = (0..advice.len()).map(|_| None).collect();
    let mut fixed_annotations: Vec<Option<&str>> = (0..fixed.len()).map(|_| None).collect();
    let mut instance_annotations: Vec<Option<&str>> = (0..instance.len()).map(|_| None).collect();

    let regions = prover.regions();

    for region in regions {
        for (col, name) in region.annotations().iter() {
            match col.column_type() {
                halo2_proofs::plonk::Any::Advice(_) => advice_annotations[col.index()] = Some(name),
                halo2_proofs::plonk::Any::Fixed => fixed_annotations[col.index()] = Some(name),
                halo2_proofs::plonk::Any::Instance => {
                    instance_annotations[col.index()] = Some(name)
                }
            }
        }
    }

    (advice_annotations, fixed_annotations, instance_annotations)
}
