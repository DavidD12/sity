use super::*;

use sealed::sealed;
use std::marker::PhantomData;

#[sealed]
pub trait ScaleFactor: Copy {
    const PREFIX: i32;
    const SYMBOL: &'static str;
    const EXPONENT: i32;
}

#[derive(Copy, Clone, Debug)]
pub struct Scale<P: Prefix, E: Exponent> {
    prefix: PhantomData<P>,
    exponent: PhantomData<E>,
}
#[sealed]
impl<P: Prefix, E: Exponent> ScaleFactor for Scale<P, E> {
    const PREFIX: i32 = P::PREFIX;
    const SYMBOL: &'static str = P::SYMBOL;
    const EXPONENT: i32 = E::EXPONENT;
}

pub fn pretty<S: ScaleFactor>() -> String {
    if S::EXPONENT == 0 {
        "".to_string()
    } else {
        format!("{}{}", S::SYMBOL, pretty_exponent(S::EXPONENT))
    }
}

//------------------------- Mul -------------------------
#[sealed]
pub trait MulScale<S: ScaleFactor> {
    type Output: ScaleFactor;
}

#[sealed]
impl<P, E, E1> MulScale<Scale<P, E1>> for Scale<P, E>
where
    P: Prefix,
    E: Exponent + MulExp<E1>,
    E1: Exponent,
{
    type Output = Scale<P, E::Output>;
}

//------------------------- Div -------------------------
#[sealed]
pub trait DivScale<S: ScaleFactor> {
    type Output: ScaleFactor;
}

#[sealed]
impl<P, E, E1> DivScale<Scale<P, E1>> for Scale<P, E>
where
    P: Prefix,
    E: Exponent + DivExp<E1>,
    E1: Exponent,
{
    type Output = Scale<P, E::Output>;
}

//------------------------- Pow -------------------------

pub trait PowScale<const N: i32> {
    type Output: ScaleFactor;
}

impl<P, E, const N: i32> PowScale<N> for Scale<P, E>
where
    P: Prefix,
    E: Exponent + PowExp<N>,
{
    type Output = Scale<P, E::Output>;
}

//------------------------- Root -------------------------

pub trait RootScale<const N: i32> {
    type Output: ScaleFactor;
}

impl<P, E, const N: i32> RootScale<N> for Scale<P, E>
where
    P: Prefix,
    E: Exponent + RootExp<N>,
{
    type Output = Scale<P, E::Output>;
}
