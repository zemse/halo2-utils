use halo2_utils::{
    assignments_printer, example_circuit::FactorisationCircuit,
    halo2_proofs::halo2curves::bn256::Fr,
};

fn main() {
    let circuit = FactorisationCircuit {
        a: Fr::from(2),
        b: Fr::from(3),
        _marker: std::marker::PhantomData,
    };

    assignments_printer::print_all(4, &circuit, None);
}

// ╭────────────────┬──────────────┬─────────────┬──────────────────╮
// │ unnamed advice │ advice colm  │ my selector │ unnamed instance │
// ├────────────────┼──────────────┼─────────────┼──────────────────┤
// │ Unassigned     │ 2            │ 1           │ 6                │
// │ Unassigned     │ 3            │ 0           │ 0                │
// │ Unassigned     │ 6            │ 0           │ 0                │
// │ Unassigned     │ Unassigned   │ 0           │ 0                │
// │ Unassigned     │ Unassigned   │ 0           │ 0                │
// │ Unassigned     │ Unassigned   │ 0           │ 0                │
// │ Unassigned     │ Unassigned   │ 0           │ 0                │
// │ Unassigned     │ Unassigned   │ 0           │ 0                │
// │ Unassigned     │ Unassigned   │ 0           │ 0                │
// │ Unassigned     │ Unassigned   │ 0           │ 0                │
// │ Poisoned(10)   │ Poisoned(10) │ 0           │ 0                │
// ╰────────────────┴──────────────┴─────────────┴──────────────────╯
