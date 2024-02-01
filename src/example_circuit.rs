use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, Instance, Selector},
    poly::Rotation,
};

use crate::{CircuitExt, FieldExt};

#[derive(Clone)]
pub struct FactorisationConfig {
    selector: Selector,
    #[allow(dead_code)]
    advice_temp: Column<Advice>,
    advice: Column<Advice>,
    instance: Column<Instance>,
}

#[derive(Clone, Default, Debug)]
pub struct FactorisationCircuit<F: FieldExt> {
    pub a: F,
    pub b: F,
    pub _marker: PhantomData<F>,
}

impl<F: FieldExt> Circuit<F> for FactorisationCircuit<F> {
    type Config = FactorisationConfig;

    type FloorPlanner = SimpleFloorPlanner;

    #[cfg(feature = "circuit-params")]
    type Params = ();

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut halo2_proofs::plonk::ConstraintSystem<F>) -> Self::Config {
        let selector = meta.selector();
        let advice_temp = meta.advice_column();
        let advice = meta.advice_column();
        let instance = meta.instance_column();

        meta.enable_equality(advice);
        meta.enable_equality(instance);

        meta.create_gate("product check", |meta| {
            let s = meta.query_selector(selector);
            let a = meta.query_advice(advice, Rotation::cur());
            let b = meta.query_advice(advice, Rotation::next());
            let product = meta.query_advice(advice, Rotation(2));
            vec![s * (a * b - product)]
        });

        Self::Config {
            selector,
            advice_temp,
            advice,
            instance,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl halo2_proofs::circuit::Layouter<F>,
    ) -> Result<(), halo2_proofs::plonk::Error> {
        let product_cell = layouter.assign_region(
            || "region main",
            |mut region| {
                region.name_column(|| "advice colm", config.advice);
                // region.name_column(|| "my selector", config.selector);

                config.selector.enable(&mut region, 0)?;
                let a_cell = region.assign_advice(
                    || "assign advice a",
                    config.advice,
                    0,
                    || Value::known(self.a),
                )?;

                let b_cell = region.assign_advice(
                    || "assign advice",
                    config.advice,
                    1,
                    || Value::known(self.b),
                )?;

                let product = a_cell.value().copied() * b_cell.value();
                region.assign_advice(|| "assign product", config.advice, 2, || product)
            },
        )?;

        layouter.constrain_instance(product_cell.cell(), config.instance, 0)
    }
}

impl<F: FieldExt> CircuitExt<F> for FactorisationCircuit<F> {
    fn instances(&self) -> Vec<Vec<F>> {
        vec![vec![self.a * self.b]]
    }
}
