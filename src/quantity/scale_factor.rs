use super::*;

use sealed::sealed;
use std::marker::PhantomData;

#[sealed]
pub trait ScaleFactor: Copy + std::fmt::Debug {
    const PREFIX: i32;
    const SYMBOL: &'static str;
    const EXPONENT: i32;
}

#[derive(Copy, Clone, Debug)]
pub struct Scale<P: Prefix, const N: i32> {
    prefix: PhantomData<P>,
}
#[sealed]
impl<P: Prefix, const N: i32> ScaleFactor for Scale<P, N> {
    const PREFIX: i32 = P::PREFIX;
    const SYMBOL: &'static str = P::SYMBOL;
    const EXPONENT: i32 = N;
}

pub fn pretty<S: ScaleFactor>(unit: &str) -> String {
    match S::EXPONENT {
        0 => "".to_string(),
        1 => format!("{}{}", S::SYMBOL, unit),
        2 => format!("{}{}²", S::SYMBOL, unit),
        3 => format!("{}{}³", S::SYMBOL, unit),
        x => format!("{}{}{}", S::SYMBOL, unit, x),
    }
}

//------------------------- Mul -------------------------
pub trait MulScale<S: ScaleFactor> {
    type Output: ScaleFactor;
}

//------------------------- Div -------------------------
pub trait DivScale<S: ScaleFactor> {
    type Output: ScaleFactor;
}

//------------------------- Pow -------------------------
pub trait PowScale<const N: i32> {
    type Output: ScaleFactor;
}

//------------------------- Root -------------------------
pub trait RootScale<const N: i32> {
    type Output: ScaleFactor;
}
