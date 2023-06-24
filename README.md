# halo2 utils

some basic utils to slightly improve dx with vanila [pse/halo2](https://github.com/privacy-scaling-explorations/halo2).

## generate layout diagrams

abstracts some dependencies and auto estimates value of k.

```rust
use halo2_utils::LayoutPrinter;

fn main() {
    let circuit = MyCircuit::<Fr>::default();
    LayoutPrinter::from(&circuit).print();
}
```

![example layout](./MyCircuit-layout.png)

## real prover

abstracts r/w kzg params from local files, generating instances, value of k.

```rust
use halo2_utils::RealProver;

fn main() {
    // implements halo2_proofs::plonk::Circuit and halo2_utils::CircuitExt
    let circuit = FactorizationCircuit {
        a: Fr::from(3),
        b: Fr::from(7),
        _marker: PhantomData,
    };

    // generate proofs
    let mut prover = RealProver::from(circuit);
    let (proof, public_inputs) = prover.run(/* write_to_file: */ true).unwrap();

    // verify proofs
    let verifier = prover.verifier();
    let success = verifier.run(proof, public_inputs);

    // yul verifier
    let code = verifier.generate_yul(/* write_to_file: */ true).unwrap();
}
```