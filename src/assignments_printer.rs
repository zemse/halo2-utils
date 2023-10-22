use std::{iter, ops::IndexMut};

use ethers::types::U256;
use halo2_proofs::{
    dev::{CellValue, MockProver},
    halo2curves::ff,
};

use crate::{CircuitExt, FieldExt};

use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Modify, Style},
};

#[derive(Debug)]
enum Column {
    Advice(usize),
    Fixed(usize),
    Instance(usize),
    Selector(usize),
}

pub fn print<F: FieldExt + ff::PrimeField, C: CircuitExt<F>>(
    k: u32,
    circuit: &C,
    columns_to_print: Vec<&str>,
) -> Result<(), halo2_proofs::plonk::Error>
where
    F::Repr: Sized + IndexMut<usize>,
{
    // let table = Table::new(&columns_to_print);
    let mut table = Builder::default();

    let header =
        iter::once("row".to_string()).chain(columns_to_print.iter().map(|s| s.to_string()));
    table.set_header(header);

    let (advice_annotations, fixed_annotations, instance_annotations, selector_annotations) =
        circuit.annotations();
    let mut col_indexes = vec![];
    'col: for col_name in &columns_to_print {
        for (i, advice_col_name) in advice_annotations.iter().enumerate() {
            if col_name == advice_col_name {
                col_indexes.push(Column::Advice(i));
                continue 'col;
            }
        }
        for (i, fixed_col_name) in fixed_annotations.iter().enumerate() {
            if col_name == fixed_col_name {
                col_indexes.push(Column::Fixed(i));
                continue 'col;
            }
        }
        for (i, instance_col_name) in instance_annotations.iter().enumerate() {
            if col_name == instance_col_name {
                col_indexes.push(Column::Instance(i));
                continue 'col;
            }
        }
        for (i, selector_col_name) in selector_annotations.iter().enumerate() {
            if col_name == selector_col_name {
                col_indexes.push(Column::Selector(i));
                continue 'col;
            }
        }
    }

    // table.push_record(
    //     iter::once("-".to_string()).chain(col_indexes.iter().map(|c| format!("{:?}", c))),
    // );

    let prover: MockProver<F> = MockProver::run(k, circuit, circuit.instances())?;

    let range = prover.usable_rows();

    let advice = prover.advice();
    let fixed = prover.fixed();
    let instance = prover.instance();
    let selectors = prover.selectors();

    for row_id in range.start..=range.end {
        table.push_record(iter::once(row_id.to_string()).chain(col_indexes.iter().map(
            |c| match c {
                Column::Advice(i) => format_cell_value(advice[*i][row_id]),
                Column::Fixed(i) => format_cell_value(fixed[*i][row_id]),
                Column::Instance(i) => format_value(instance[*i][row_id]),
                Column::Selector(i) => {
                    if selectors[*i][row_id] {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    }
                }
            },
        )));
    }

    let str = table
        .build()
        .with(Style::rounded())
        .with(Modify::new(Rows::new(1..)).with(Alignment::left()))
        .to_string();

    println!("{}", str);

    Ok(())
}

fn format_cell_value<F: FieldExt + ff::PrimeField>(value: CellValue<F>) -> String {
    match value {
        CellValue::Unassigned => "Unassigned".to_string(),
        CellValue::Assigned(f) => format_value(f),
        CellValue::Poison(v) => format!("Poisoned({})", v),
    }
}
fn format_value<F: FieldExt + ff::PrimeField>(f: F) -> String {
    let v = f.to_repr();
    let v = v.as_ref();
    let v = U256::from_little_endian(v);
    format!("{:?}", v)
}
