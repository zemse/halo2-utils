# halo2 utils

```rust
use halo2_utils::Printer;

fn main() {
    let circuit = MyCircuit::<Fr>::default();
    Printer::from(4, &circuit).print();
}
```