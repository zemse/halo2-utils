use halo2_utils::{example_circuit::FactorisationCircuit, halo2_proofs::halo2curves::bn256::Fr};

fn main() {
    let circuit = FactorisationCircuit {
        a: Fr::from(2),
        b: Fr::from(3),
        _marker: std::marker::PhantomData,
    };

    halo2_utils::assignments::print_all(&circuit, Some(4));
}

// output
//
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
