use halo2_proofs_v2022_08_19::{arithmetic::Group, halo2curves::bn256::Fr};

#[cfg(any(feature = "v030-halo2", feature = "latest-halo2"))]
use crate::halo2_proofs::halo2curves::ff::FromUniformBytes;
#[cfg(any(feature = "v030-halo2", feature = "latest-halo2"))]
use halo2_proofs_v2022_08_19::halo2curves::bn256::Fp;

#[cfg(any(feature = "v030-halo2", feature = "latest-halo2"))]
pub trait RawField: crate::halo2_proofs::arithmetic::Field + FromUniformBytes<64> + Ord {}

#[cfg(not(any(feature = "v030-halo2", feature = "latest-halo2")))]
pub trait RawField: crate::halo2_proofs::arithmetic::FieldExt + Group + From<u64> + Ord {}
impl RawField for Fr {}

#[cfg(any(feature = "v030-halo2", feature = "latest-halo2"))]
impl RawField for Fp {}

pub trait FieldExt: RawField + From<u64> {}

impl FieldExt for Fr {}

#[cfg(any(feature = "v030-halo2", feature = "latest-halo2"))]
impl FieldExt for Fp {}
