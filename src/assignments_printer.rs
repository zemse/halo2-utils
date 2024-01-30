use std::{iter, ops::IndexMut};

use ethers::types::{BigEndianHash, H256, U256};
use halo2_proofs::{
    dev::{CellValue, MockProver},
    halo2curves::ff,
};

use crate::{instance_value, CircuitExt, FieldExt};

use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Modify, Style},
};

#[derive(Debug, PartialEq)]
pub enum Column {
    Advice(usize),
    Fixed(usize),
    Instance(usize),
    Selector(usize),
}

pub fn print<F: FieldExt + ff::PrimeField, C: CircuitExt<F>>(
    k: u32,
    circuit: &C,
    columns_to_print: Vec<&str>,
) where
    F::Repr: Sized + IndexMut<usize>,
{
    let prover: MockProver<F> = MockProver::run(k, circuit, circuit.instances()).unwrap();

    let mut table = Builder::default();

    let header =
        iter::once("row".to_string()).chain(columns_to_print.iter().map(|s| s.to_string()));
    table.set_header(header);

    let (advice_annotations, fixed_annotations, instance_annotations) = get_annotations(&prover);

    let mut col_indexes = vec![];
    'col: for col_name in &columns_to_print {
        for (i, advice_col_name) in advice_annotations.iter().enumerate() {
            if advice_col_name.is_some() && col_name == advice_col_name.as_ref().unwrap() {
                col_indexes.push(Column::Advice(i));
                continue 'col;
            }
        }
        for (i, fixed_col_name) in fixed_annotations.iter().enumerate() {
            if fixed_col_name.is_some() && col_name == fixed_col_name.as_ref().unwrap() {
                col_indexes.push(Column::Fixed(i));
                continue 'col;
            }
        }
        for (i, instance_col_name) in instance_annotations.iter().enumerate() {
            if instance_col_name.is_some() && col_name == instance_col_name.as_ref().unwrap() {
                col_indexes.push(Column::Instance(i));
                continue 'col;
            }
        }
        panic!("assignments_printer: column '{}' not found", col_name);
    }

    // table.push_record(
    //     iter::once("-".to_string()).chain(col_indexes.iter().map(|c| format!("{:?}", c))),
    // );

    let range = prover.usable_rows();

    let advice = prover.advice();
    let fixed = prover.fixed();
    let instance = prover.instance();

    for row_id in range.start..=range.end {
        table.push_record(iter::once(row_id.to_string()).chain(col_indexes.iter().map(
            |c| match c {
                Column::Advice(i) => format_cell_value(advice[*i][row_id]),
                Column::Fixed(i) => format_cell_value(fixed[*i][row_id]),
                Column::Instance(i) => format_value(instance_value(&instance[*i][row_id])),
                Column::Selector(_) => unreachable!(),
            },
        )));
    }

    let str = table
        .build()
        .with(Style::rounded())
        .with(Modify::new(Rows::new(1..)).with(Alignment::left()))
        .to_string();

    println!("{}", str);
}

pub fn print_all<F: FieldExt + ff::PrimeField, C: CircuitExt<F>>(
    k: u32,
    circuit: &C,
    skip: Option<Vec<Column>>,
) where
    F::Repr: Sized + IndexMut<usize>,
{
    let prover: MockProver<F> = MockProver::run(k, circuit, circuit.instances()).unwrap();

    let range = prover.usable_rows();

    let advice = prover.advice();
    let fixed = prover.fixed();
    let instance = prover.instance();

    let (advice_annotations, fixed_annotations, instance_annotations) =
        // circuit.annotations();
        get_annotations(&prover);

    let mut header: Vec<&str> = vec![];
    for (i, _) in advice.iter().enumerate() {
        let should_skip = skip.as_ref().map(|skip| skip.contains(&Column::Advice(i)));
        if should_skip.unwrap_or(false) {
            continue;
        }
        if let Some(name) = advice_annotations[i] {
            header.push(name);
        } else {
            header.push("unnamed advice");
        }
    }
    for (i, _) in fixed.iter().enumerate() {
        let should_skip = skip.as_ref().map(|skip| skip.contains(&Column::Fixed(i)));
        if should_skip.unwrap_or(false) {
            continue;
        }
        if let Some(name) = fixed_annotations[i] {
            header.push(name);
        } else {
            header.push("unnamed fixed");
        }
    }
    for (i, _) in instance.iter().enumerate() {
        let should_skip = skip
            .as_ref()
            .map(|skip| skip.contains(&Column::Instance(i)));
        if should_skip.unwrap_or(false) {
            continue;
        }
        if let Some(name) = instance_annotations[i] {
            header.push(name);
        } else {
            header.push("unnamed instance");
        }
    }

    let mut table = Builder::default();
    table.set_header(header);

    for row_id in range.start..=range.end {
        let mut row_data = vec![];
        for (i, col) in advice.iter().enumerate() {
            let should_skip = skip.as_ref().map(|skip| skip.contains(&Column::Advice(i)));
            if should_skip.unwrap_or(false) {
                continue;
            }
            row_data.push(format_cell_value(col[row_id]));
        }
        for (i, col) in fixed.iter().enumerate() {
            let should_skip = skip.as_ref().map(|skip| skip.contains(&Column::Fixed(i)));
            if should_skip.unwrap_or(false) {
                continue;
            }
            row_data.push(format_cell_value(col[row_id]));
        }
        for (i, col) in instance.iter().enumerate() {
            let should_skip = skip
                .as_ref()
                .map(|skip| skip.contains(&Column::Instance(i)));
            if should_skip.unwrap_or(false) {
                continue;
            }
            row_data.push(format_value(instance_value(&col[row_id])));
        }

        table.push_record(row_data);
    }

    let str = table
        .build()
        .with(Style::rounded())
        .with(Modify::new(Rows::new(1..)).with(Alignment::left()))
        .to_string();

    println!("{}", str);
}

fn format_cell_value<F: FieldExt + ff::PrimeField>(value: CellValue<F>) -> String {
    match value {
        CellValue::Unassigned => "Unassigned".to_string(),
        CellValue::Assigned(f) => format_value(f),
        CellValue::Poison(v) => format!("Poisoned({})", v),
    }
}

// fn format_instance_value<F: FieldExt + ff::PrimeField>(value: InstanceValue<F>) -> String {
//     match value {
//         InstanceValue::Assigned(f) => format_value(f),
//         InstanceValue::Padding => "Padding".to_string(),
//     }
// }

fn format_value<F: FieldExt + ff::PrimeField>(f: F) -> String {
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
fn get_annotations<F: FieldExt + ff::PrimeField>(
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
        for (col, name) in region.annotations.iter() {
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
