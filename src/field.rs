use halo2_proofs::halo2curves::{bn256::Fr, ff::FromUniformBytes, pasta::Fp};

pub trait RawField: halo2_proofs::arithmetic::Field + FromUniformBytes<64> + Ord {}
impl RawField for Fr {}
impl RawField for Fp {}

pub trait FieldExt: RawField + From<u64> {}

impl FieldExt for Fr {}
impl FieldExt for Fp {}
