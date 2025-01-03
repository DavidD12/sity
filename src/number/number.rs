//------------------------- HasValue -------------------------

pub trait Scalar:
    Number
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Div<Self, Output = Self>
    + std::ops::MulAssign<Self>
    + std::ops::DivAssign<Self>
    + HasValue<Output = Self>
{
}

pub trait HasValue {
    type Output: Scalar;
    fn value(self) -> Self::Output;
}

//------------------------- Base -------------------------

use std::process::Output;

use crate::Prefix;

pub trait ToBase {
    fn to_base<P1: Prefix, P2: Prefix, const N: i32>(&self) -> Self;
}

//------------------------- Convert -------------------------

pub trait Convert {
    type Output;
    fn convert(&self) -> Output;
}

//------------------------- Number -------------------------

pub trait Number:
    Copy + std::fmt::Debug + Default
    + HasValue
    + PartialEq
    + PartialOrd
    // + std::ops::Neg<Output = Self>
    + std::ops::Add<Self, Output = Self>
    + std::ops::AddAssign<Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::SubAssign<Self>
    + std::fmt::Display
{
    const ZERO: Self;
    const ONE: Self;
    const EPSILON: Self;

    fn abs(self) -> Self;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn floor(self) -> Self;
    fn round(self) -> Self;
    fn ceil(self) -> Self;
    fn trunc(self) -> Self;

    fn same(self, other: Self) -> bool {
        if self >= other {
            self - other <= Self::EPSILON
        }
        else {
            other - self <= Self::EPSILON
        }
    }
}

//------------------------- From Value -------------------------

pub trait FromValue<T: Scalar> {
    fn from_value(value: T) -> Self;
}

//------------------------- Pow -------------------------

pub trait Pow2: Number {
    type Output;
    fn pow2(&self) -> <Self as Pow2>::Output;
}

pub trait Pow3: Number {
    type Output;
    fn pow3(&self) -> <Self as Pow3>::Output;
}

pub trait Pow4: Number {
    type Output;
    fn pow4(&self) -> <Self as Pow4>::Output;
}

//------------------------- Root -------------------------

pub trait Root2: Number {
    type Output;
    fn root2(&self) -> <Self as Root2>::Output;
}

pub trait Root3: Number {
    type Output;
    fn root3(&self) -> <Self as Root3>::Output;
}

pub trait Root4: Number {
    type Output;
    fn root4(&self) -> <Self as Root4>::Output;
}
