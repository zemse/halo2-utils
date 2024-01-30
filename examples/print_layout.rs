use halo2_utils::{
    example_circuit::FactorisationCircuit, halo2_proofs::halo2curves::bn256::Fr, LayoutPrinter,
};

fn main() {
    let circuit = FactorisationCircuit::<Fr>::default();
    LayoutPrinter::from(&circuit).print();
}
