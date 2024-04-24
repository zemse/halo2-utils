#[cfg(any(feature = "latest-halo2", feature = "v030-halo2"))]
use halo2_utils::{example_circuit::FactorisationCircuit, halo2_proofs::halo2curves::bn256::Fr};

fn main() {
    #[cfg(any(feature = "latest-halo2", feature = "v030-halo2"))]
    let circuit = FactorisationCircuit {
        a: Fr::from(2),
        b: Fr::from(3),
        _marker: std::marker::PhantomData,
    };

    #[cfg(any(feature = "latest-halo2", feature = "v030-halo2"))]
    halo2_utils::assignments::print_all(&circuit, Some(4), None);

    #[cfg(not(any(feature = "latest-halo2", feature = "v030-halo2")))]
    panic!("This example is only supported under the 'latest-halo2' or 'v030-halo2' feature")
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
