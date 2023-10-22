use halo2_proofs::plonk::Expression;

use crate::FieldExt;

/// Trait that implements functionality to get a scalar from
/// commonly used types.
pub trait Scalar<F: FieldExt> {
    /// Returns a scalar for the type.
    fn scalar(&self) -> F;
}

/// Implementation trait `Scalar` for type able to be casted to u64
#[macro_export]
macro_rules! impl_scalar {
    ($type:ty) => {
        impl<F: $crate::FieldExt> $crate::Scalar<F> for $type {
            #[inline]
            fn scalar(&self) -> F {
                F::from(*self as u64)
            }
        }
    };
    ($type:ty, $method:path) => {
        impl<F: eth_types::Field> $crate::util::Scalar<F> for $type {
            #[inline]
            fn scalar(&self) -> F {
                F::from($method(self) as u64)
            }
        }
    };
}

/// Trait that implements functionality to get a constant expression from
/// commonly used types.
pub trait Expr<F: FieldExt> {
    /// Returns an expression for the type.
    fn expr(&self) -> Expression<F>;
}

/// Implementation trait `Expr` for type able to be casted to u64
#[macro_export]
macro_rules! impl_expr {
    ($type:ty) => {
        $crate::impl_scalar!($type);
        impl<F: $crate::FieldExt> $crate::Expr<F> for $type {
            #[inline]
            fn expr(&self) -> Expression<F> {
                Expression::Constant(F::from(*self as u64))
            }
        }
    };
    ($type:ty, $method:path) => {
        $crate::impl_scalar!($type, $method);
        impl<F: eth_types::Field> $crate::util::Expr<F> for $type {
            #[inline]
            fn expr(&self) -> Expression<F> {
                Expression::Constant(F::from($method(self) as u64))
            }
        }
    };
}

impl_expr!(bool);
impl_expr!(u8);
impl_expr!(u64);
impl_expr!(i32);
impl_expr!(usize);
impl_expr!(isize);
