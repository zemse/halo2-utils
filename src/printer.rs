use std::marker::PhantomData;

use halo2_proofs::{arithmetic::Field, dev::CircuitLayout, plonk::Circuit};
use plotters::prelude::*;

pub struct Printer<'a, F: Field, ConcreteCircuit: Circuit<F>> {
    // main params
    _k: u32,
    _circuit: &'a ConcreteCircuit,
    // configurable params
    _layout: CircuitLayout,
    // _root: DrawingArea<BitMapBackend<'a>, plotters::coord::Shift>,
    // some configs
    _path: &'a str,
    _color: &'a RGBColor,
    _dimensions: (u32, u32),
    _title: &'a str,
    // markers
    _marker: PhantomData<F>,
}

impl<'a, F: Field, ConcreteCircuit: Circuit<F>> Printer<'a, F, ConcreteCircuit> {
    pub fn from(k: u32, circuit: &'a ConcreteCircuit) -> Self {
        // let root = BitMapBackend::new("simple-example-layout.png", (1024, 768)).into_drawing_area();
        Self {
            _k: k,
            _circuit: circuit,
            _layout: CircuitLayout::default()
                .mark_equality_cells(true)
                .show_equality_constraints(true)
                .show_labels(true),
            // _root: root,
            _path: "circuit-layout.png",
            _color: &WHITE,
            _dimensions: (1024, 768),
            _title: "Circuit Layout",
            _marker: PhantomData::default(),
        }
    }

    pub fn print(self) {
        let root = BitMapBackend::new(self._path, self._dimensions).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.titled(self._title, ("sans-serif", 60)).unwrap();

        self._layout.render(self._k, self._circuit, &root).unwrap();
    }

    pub fn path(mut self, path: &'a str) -> Self {
        self._path = path;
        self
    }

    pub fn color(mut self, color: &'a RGBColor) -> Self {
        self._color = color;
        self
    }

    pub fn dimensions(mut self, dimensions: (u32, u32)) -> Self {
        self._dimensions = dimensions;
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self._title = title;
        self
    }

    pub fn layout_config<CL>(mut self, layout_fn: CL) -> Self
    where
        CL: Fn(CircuitLayout) -> CircuitLayout,
    {
        self._layout = layout_fn(self._layout);
        self
    }

    // pub fn draw_config<R>(mut self, root_fn: R) -> Self
    // where
    //     R: Fn(
    //         DrawingArea<BitMapBackend<'_>, plotters::coord::Shift>,
    //     ) -> DrawingArea<BitMapBackend<'_>, plotters::coord::Shift>,
    // {
    //     self._root = root_fn(self._root);
    //     self
    // }
}

#[cfg(test)]
mod tests {
    use halo2_proofs::halo2curves::bn256::Fr;

    use super::*;
    use crate::example_circuit::MyCircuit;

    #[test]
    fn it_works() {
        let circuit = MyCircuit::<Fr>::default();
        Printer::from(4, &circuit).print();
    }
}
