use std::marker::PhantomData;

use super::*;
use sealed::sealed;

#[sealed]
pub trait Exponent: Copy {
    const VALUE: i32;
    const BASE: i32;
    const BASE_SYMBOL: &'static str;
}

#[derive(Copy, Clone, Debug)]
pub struct Exp0;
#[sealed]
impl Exponent for Exp0 {
    const VALUE: i32 = 0;
    const BASE: i32 = 0;
    const BASE_SYMBOL: &'static str = "";
}

#[derive(Copy, Clone, Debug)]
pub struct Exp1<P: Prefix> {
    prefix: PhantomData<P>,
}
#[sealed]
impl<P: Prefix> Exponent for Exp1<P> {
    const VALUE: i32 = 1;
    const BASE: i32 = P::BASE;
    const BASE_SYMBOL: &'static str = P::SYMBOL;
}

#[derive(Copy, Clone, Debug)]
pub struct Exp2<P: Prefix> {
    prefix: PhantomData<P>,
}
#[sealed]
impl<P: Prefix> Exponent for Exp2<P> {
    const VALUE: i32 = 2;
    const BASE: i32 = P::BASE;
    const BASE_SYMBOL: &'static str = P::SYMBOL;
}

#[derive(Copy, Clone, Debug)]
pub struct Exp3<P: Prefix> {
    prefix: PhantomData<P>,
}
#[sealed]
impl<P: Prefix> Exponent for Exp3<P> {
    const VALUE: i32 = 3;
    const BASE: i32 = P::BASE;
    const BASE_SYMBOL: &'static str = P::SYMBOL;
}

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct Exp_1<P: Prefix> {
    prefix: PhantomData<P>,
}
#[sealed]
impl<P: Prefix> Exponent for Exp_1<P> {
    const VALUE: i32 = -1;
    const BASE: i32 = P::BASE;
    const BASE_SYMBOL: &'static str = P::SYMBOL;
}

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct Exp_2<P: Prefix> {
    prefix: PhantomData<P>,
}
#[sealed]
impl<P: Prefix> Exponent for Exp_2<P> {
    const VALUE: i32 = -2;
    const BASE: i32 = P::BASE;
    const BASE_SYMBOL: &'static str = P::SYMBOL;
}

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct Exp_3<P: Prefix> {
    prefix: PhantomData<P>,
}
#[sealed]
impl<P: Prefix> Exponent for Exp_3<P> {
    const VALUE: i32 = -3;
    const BASE: i32 = P::BASE;
    const BASE_SYMBOL: &'static str = P::SYMBOL;
}

//------------------------- Pow2 -------------------------

pub trait Pow2Exp {
    type Output: Exponent;
}

impl Pow2Exp for Exp0 {
    type Output = Exp0;
}

impl<P: Prefix> Pow2Exp for Exp1<P> {
    type Output = Exp2<P>;
}

//------------------------- Pow3 -------------------------

pub trait Pow3Exp {
    type Output: Exponent;
}

impl Pow3Exp for Exp0 {
    type Output = Exp0;
}

impl<P: Prefix> Pow3Exp for Exp1<P> {
    type Output = Exp3<P>;
}

//------------------------- Root2 -------------------------

pub trait Root2Exp {
    type Output: Exponent;
}

impl Root2Exp for Exp0 {
    type Output = Exp0;
}

impl<P: Prefix> Root2Exp for Exp2<P> {
    type Output = Exp1<P>;
}

//------------------------- Root3 -------------------------

pub trait Root3Exp {
    type Output: Exponent;
}

impl Root3Exp for Exp0 {
    type Output = Exp0;
}

impl<P: Prefix> Root3Exp for Exp3<P> {
    type Output = Exp1<P>;
}

//------------------------- Mul -------------------------

pub trait MulExp<E: Exponent> {
    type Output: Exponent;
}

impl<P: Prefix> MulExp<Exp0> for Exp_3<P> {
    type Output = Exp_3<P>;
}
impl<P: Prefix> MulExp<Exp1<P>> for Exp_3<P> {
    type Output = Exp_2<P>;
}
impl<P: Prefix> MulExp<Exp2<P>> for Exp_3<P> {
    type Output = Exp_1<P>;
}
impl<P: Prefix> MulExp<Exp3<P>> for Exp_3<P> {
    type Output = Exp0;
}

impl<P: Prefix> MulExp<Exp_1<P>> for Exp_2<P> {
    type Output = Exp_3<P>;
}
impl<P: Prefix> MulExp<Exp0> for Exp_2<P> {
    type Output = Exp_2<P>;
}
impl<P: Prefix> MulExp<Exp1<P>> for Exp_2<P> {
    type Output = Exp_1<P>;
}
impl<P: Prefix> MulExp<Exp2<P>> for Exp_2<P> {
    type Output = Exp0;
}
impl<P: Prefix> MulExp<Exp3<P>> for Exp_2<P> {
    type Output = Exp1<P>;
}

impl<P: Prefix> MulExp<Exp_2<P>> for Exp_1<P> {
    type Output = Exp_3<P>;
}
impl<P: Prefix> MulExp<Exp_1<P>> for Exp_1<P> {
    type Output = Exp_2<P>;
}
impl<P: Prefix> MulExp<Exp0> for Exp_1<P> {
    type Output = Exp_1<P>;
}
impl<P: Prefix> MulExp<Exp1<P>> for Exp_1<P> {
    type Output = Exp0;
}
impl<P: Prefix> MulExp<Exp2<P>> for Exp_1<P> {
    type Output = Exp1<P>;
}
impl<P: Prefix> MulExp<Exp3<P>> for Exp_1<P> {
    type Output = Exp2<P>;
}

impl<E: Exponent> MulExp<E> for Exp0 {
    type Output = E;
}

impl<P: Prefix> MulExp<Exp_3<P>> for Exp1<P> {
    type Output = Exp_2<P>;
}
impl<P: Prefix> MulExp<Exp_2<P>> for Exp1<P> {
    type Output = Exp_1<P>;
}
impl<P: Prefix> MulExp<Exp_1<P>> for Exp1<P> {
    type Output = Exp0;
}
impl<P: Prefix> MulExp<Exp0> for Exp1<P> {
    type Output = Exp1<P>;
}
impl<P: Prefix> MulExp<Exp1<P>> for Exp1<P> {
    type Output = Exp2<P>;
}
impl<P: Prefix> MulExp<Exp2<P>> for Exp1<P> {
    type Output = Exp3<P>;
}

impl<P: Prefix> MulExp<Exp_3<P>> for Exp2<P> {
    type Output = Exp_1<P>;
}
impl<P: Prefix> MulExp<Exp_2<P>> for Exp2<P> {
    type Output = Exp0;
}
impl<P: Prefix> MulExp<Exp_1<P>> for Exp2<P> {
    type Output = Exp1<P>;
}
impl<P: Prefix> MulExp<Exp0> for Exp2<P> {
    type Output = Exp2<P>;
}
impl<P: Prefix> MulExp<Exp1<P>> for Exp2<P> {
    type Output = Exp3<P>;
}

impl<P: Prefix> MulExp<Exp_3<P>> for Exp3<P> {
    type Output = Exp0;
}
impl<P: Prefix> MulExp<Exp_2<P>> for Exp3<P> {
    type Output = Exp1<P>;
}
impl<P: Prefix> MulExp<Exp_1<P>> for Exp3<P> {
    type Output = Exp2<P>;
}
impl<P: Prefix> MulExp<Exp0> for Exp3<P> {
    type Output = Exp3<P>;
}

//------------------------- Div -------------------------

pub trait DivExp<E: Exponent> {
    type DivExpOutput: Exponent;
}

impl<P: Prefix> DivExp<Exp_3<P>> for Exp_3<P> {
    type DivExpOutput = Exp0;
}
impl<P: Prefix> DivExp<Exp_2<P>> for Exp_3<P> {
    type DivExpOutput = Exp_1<P>;
}
impl<P: Prefix> DivExp<Exp_1<P>> for Exp_3<P> {
    type DivExpOutput = Exp_2<P>;
}
impl<P: Prefix> DivExp<Exp0> for Exp_3<P> {
    type DivExpOutput = Exp_3<P>;
}

impl<P: Prefix> DivExp<Exp_3<P>> for Exp_2<P> {
    type DivExpOutput = Exp1<P>;
}
impl<P: Prefix> DivExp<Exp_2<P>> for Exp_2<P> {
    type DivExpOutput = Exp0;
}
impl<P: Prefix> DivExp<Exp_1<P>> for Exp_2<P> {
    type DivExpOutput = Exp_1<P>;
}
impl<P: Prefix> DivExp<Exp0> for Exp_2<P> {
    type DivExpOutput = Exp_2<P>;
}
impl<P: Prefix> DivExp<Exp1<P>> for Exp_2<P> {
    type DivExpOutput = Exp_3<P>;
}

impl<P: Prefix> DivExp<Exp_3<P>> for Exp_1<P> {
    type DivExpOutput = Exp2<P>;
}
impl<P: Prefix> DivExp<Exp_2<P>> for Exp_1<P> {
    type DivExpOutput = Exp1<P>;
}
impl<P: Prefix> DivExp<Exp_1<P>> for Exp_1<P> {
    type DivExpOutput = Exp0;
}
impl<P: Prefix> DivExp<Exp0> for Exp_1<P> {
    type DivExpOutput = Exp_1<P>;
}
impl<P: Prefix> DivExp<Exp1<P>> for Exp_1<P> {
    type DivExpOutput = Exp_2<P>;
}
impl<P: Prefix> DivExp<Exp2<P>> for Exp_1<P> {
    type DivExpOutput = Exp_3<P>;
}

impl<P: Prefix> DivExp<Exp_3<P>> for Exp0 {
    type DivExpOutput = Exp3<P>;
}
impl<P: Prefix> DivExp<Exp_2<P>> for Exp0 {
    type DivExpOutput = Exp2<P>;
}
impl<P: Prefix> DivExp<Exp_1<P>> for Exp0 {
    type DivExpOutput = Exp1<P>;
}
impl DivExp<Exp0> for Exp0 {
    type DivExpOutput = Exp0;
}
impl<P: Prefix> DivExp<Exp1<P>> for Exp0 {
    type DivExpOutput = Exp_1<P>;
}
impl<P: Prefix> DivExp<Exp2<P>> for Exp0 {
    type DivExpOutput = Exp_2<P>;
}
impl<P: Prefix> DivExp<Exp3<P>> for Exp0 {
    type DivExpOutput = Exp_3<P>;
}

impl<P: Prefix> DivExp<Exp_2<P>> for Exp1<P> {
    type DivExpOutput = Exp3<P>;
}
impl<P: Prefix> DivExp<Exp_1<P>> for Exp1<P> {
    type DivExpOutput = Exp2<P>;
}
impl<P: Prefix> DivExp<Exp0> for Exp1<P> {
    type DivExpOutput = Exp1<P>;
}
impl<P: Prefix> DivExp<Exp1<P>> for Exp1<P> {
    type DivExpOutput = Exp0;
}
impl<P: Prefix> DivExp<Exp2<P>> for Exp1<P> {
    type DivExpOutput = Exp_1<P>;
}
impl<P: Prefix> DivExp<Exp3<P>> for Exp1<P> {
    type DivExpOutput = Exp_2<P>;
}

impl<P: Prefix> DivExp<Exp_1<P>> for Exp2<P> {
    type DivExpOutput = Exp3<P>;
}
impl<P: Prefix> DivExp<Exp0> for Exp2<P> {
    type DivExpOutput = Exp2<P>;
}
impl<P: Prefix> DivExp<Exp1<P>> for Exp2<P> {
    type DivExpOutput = Exp1<P>;
}
impl<P: Prefix> DivExp<Exp2<P>> for Exp2<P> {
    type DivExpOutput = Exp0;
}
impl<P: Prefix> DivExp<Exp3<P>> for Exp2<P> {
    type DivExpOutput = Exp_1<P>;
}

impl<P: Prefix> DivExp<Exp0> for Exp3<P> {
    type DivExpOutput = Exp3<P>;
}
impl<P: Prefix> DivExp<Exp1<P>> for Exp3<P> {
    type DivExpOutput = Exp2<P>;
}
impl<P: Prefix> DivExp<Exp2<P>> for Exp3<P> {
    type DivExpOutput = Exp1<P>;
}
impl<P: Prefix> DivExp<Exp3<P>> for Exp3<P> {
    type DivExpOutput = Exp0;
}

//------------------------- Neg -------------------------

pub trait InvExp {
    type NegOutput: Exponent;
}

impl<P: Prefix> InvExp for Exp_3<P> {
    type NegOutput = Exp3<P>;
}
impl<P: Prefix> InvExp for Exp_2<P> {
    type NegOutput = Exp2<P>;
}
impl<P: Prefix> InvExp for Exp_1<P> {
    type NegOutput = Exp1<P>;
}
impl InvExp for Exp0 {
    type NegOutput = Exp0;
}
impl<P: Prefix> InvExp for Exp1<P> {
    type NegOutput = Exp_1<P>;
}
impl<P: Prefix> InvExp for Exp2<P> {
    type NegOutput = Exp_2<P>;
}
impl<P: Prefix> InvExp for Exp3<P> {
    type NegOutput = Exp_3<P>;
}
