use std::{fmt::Debug, marker::PhantomData};

use halo2_proofs::{arithmetic::Field, dev::CircuitLayout, plonk::Circuit};
use plotters::prelude::*;

use crate::{derive_circuit_name, utils::derive_k};

pub struct LayoutPrinter<'a, F: Field, ConcreteCircuit: Circuit<F>> {
    // main params
    _k: u32,
    _circuit: &'a ConcreteCircuit,
    // configurable params
    _layout: CircuitLayout,
    // _root: DrawingArea<BitMapBackend<'a>, plotters::coord::Shift>,
    // some configs
    _path: String,
    _color: &'a RGBColor,
    _dimensions: (u32, u32),
    _title: String,
    // markers
    _marker: PhantomData<F>,
}

impl<'a, F: Field, ConcreteCircuit: Circuit<F> + Debug> LayoutPrinter<'a, F, ConcreteCircuit> {
    pub fn from(circuit: &'a ConcreteCircuit) -> Self {
        let circuit_name = derive_circuit_name(circuit);
        Self {
            _k: derive_k::<F, ConcreteCircuit>(),
            _circuit: circuit,
            _layout: CircuitLayout::default()
                .mark_equality_cells(true)
                .show_equality_constraints(true)
                .show_labels(true),
            // _root: root,
            _path: format!("{}-layout.png", circuit_name),
            _color: &WHITE,
            _dimensions: (1024, 768),
            _title: format!("{} Layout", circuit_name),
            _marker: PhantomData,
        }
    }

    pub fn print(self) {
        let root = BitMapBackend::new(self._path.as_str(), self._dimensions).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root
            .titled(
                format!("{} (k={})", self._title, self._k).as_str(),
                ("sans-serif", 60),
            )
            .unwrap();

        self._layout.render(self._k, self._circuit, &root).unwrap();
    }

    pub fn degree(mut self, k: u32) -> Self {
        self._k = k;
        self
    }

    pub fn path(mut self, path: &'a str) -> Self {
        self._path = String::from(path);
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
        self._title = String::from(title);
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
        LayoutPrinter::from(&circuit).print();
    }
}
