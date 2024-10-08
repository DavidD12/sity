use sealed::sealed;

#[sealed]
pub trait Exponent: Copy {
    fn value() -> isize;
    fn pretty() -> &'static str;
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct Exp_3;
#[sealed]
impl Exponent for Exp_3 {
    fn value() -> isize {
        -3
    }
    fn pretty() -> &'static str {
        "-3"
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct Exp_2;
#[sealed]
impl Exponent for Exp_2 {
    fn value() -> isize {
        -2
    }
    fn pretty() -> &'static str {
        "-2"
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct Exp_1;
#[sealed]
impl Exponent for Exp_1 {
    fn value() -> isize {
        -1
    }
    fn pretty() -> &'static str {
        "-1"
    }
}

#[derive(Copy, Clone)]
pub struct Exp0;
#[sealed]
impl Exponent for Exp0 {
    fn value() -> isize {
        0
    }
    fn pretty() -> &'static str {
        ""
    }
}

#[derive(Copy, Clone)]
pub struct Exp1;
#[sealed]
impl Exponent for Exp1 {
    fn value() -> isize {
        1
    }
    fn pretty() -> &'static str {
        ""
    }
}

#[derive(Copy, Clone)]
pub struct Exp2;
#[sealed]
impl Exponent for Exp2 {
    fn value() -> isize {
        2
    }
    fn pretty() -> &'static str {
        "2" //"²"
    }
}

#[derive(Copy, Clone)]
pub struct Exp3;
#[sealed]
impl Exponent for Exp3 {
    fn value() -> isize {
        3
    }
    fn pretty() -> &'static str {
        "3" //"³"
    }
}

//------------------------- Pow2 -------------------------

pub trait Pow2Exp {
    type Output: Exponent;
}

impl Pow2Exp for Exp0 {
    type Output = Exp0;
}

impl Pow2Exp for Exp1 {
    type Output = Exp2;
}

//------------------------- Pow3 -------------------------

pub trait Pow3Exp {
    type Output: Exponent;
}

impl Pow3Exp for Exp0 {
    type Output = Exp0;
}

impl Pow3Exp for Exp1 {
    type Output = Exp3;
}

//------------------------- Root2 -------------------------

pub trait Root2Exp {
    type Output: Exponent;
}

impl Root2Exp for Exp0 {
    type Output = Exp0;
}

impl Root2Exp for Exp2 {
    type Output = Exp1;
}

//------------------------- Root3 -------------------------

pub trait Root3Exp {
    type Output: Exponent;
}

impl Root3Exp for Exp0 {
    type Output = Exp0;
}

impl Root3Exp for Exp3 {
    type Output = Exp1;
}

//------------------------- Mul -------------------------

pub trait MulExp<E: Exponent> {
    type Output: Exponent;
}

impl MulExp<Exp0> for Exp_3 {
    type Output = Exp_3;
}
impl MulExp<Exp1> for Exp_3 {
    type Output = Exp_2;
}
impl MulExp<Exp2> for Exp_3 {
    type Output = Exp_1;
}
impl MulExp<Exp3> for Exp_3 {
    type Output = Exp0;
}

impl MulExp<Exp_1> for Exp_2 {
    type Output = Exp_3;
}
impl MulExp<Exp0> for Exp_2 {
    type Output = Exp_2;
}
impl MulExp<Exp1> for Exp_2 {
    type Output = Exp_1;
}
impl MulExp<Exp2> for Exp_2 {
    type Output = Exp0;
}
impl MulExp<Exp3> for Exp_2 {
    type Output = Exp1;
}

impl MulExp<Exp_2> for Exp_1 {
    type Output = Exp_3;
}
impl MulExp<Exp_1> for Exp_1 {
    type Output = Exp_2;
}
impl MulExp<Exp0> for Exp_1 {
    type Output = Exp_1;
}
impl MulExp<Exp1> for Exp_1 {
    type Output = Exp0;
}
impl MulExp<Exp2> for Exp_1 {
    type Output = Exp1;
}
impl MulExp<Exp3> for Exp_1 {
    type Output = Exp2;
}

impl<E: Exponent> MulExp<E> for Exp0 {
    type Output = E;
}

impl MulExp<Exp_3> for Exp1 {
    type Output = Exp_2;
}
impl MulExp<Exp_2> for Exp1 {
    type Output = Exp_1;
}
impl MulExp<Exp_1> for Exp1 {
    type Output = Exp0;
}
impl MulExp<Exp0> for Exp1 {
    type Output = Exp1;
}
impl MulExp<Exp1> for Exp1 {
    type Output = Exp2;
}
impl MulExp<Exp2> for Exp1 {
    type Output = Exp3;
}

impl MulExp<Exp_3> for Exp2 {
    type Output = Exp_1;
}
impl MulExp<Exp_2> for Exp2 {
    type Output = Exp0;
}
impl MulExp<Exp_1> for Exp2 {
    type Output = Exp1;
}
impl MulExp<Exp0> for Exp2 {
    type Output = Exp2;
}
impl MulExp<Exp1> for Exp2 {
    type Output = Exp3;
}

impl MulExp<Exp_3> for Exp3 {
    type Output = Exp0;
}
impl MulExp<Exp_2> for Exp3 {
    type Output = Exp1;
}
impl MulExp<Exp_1> for Exp3 {
    type Output = Exp2;
}
impl MulExp<Exp0> for Exp3 {
    type Output = Exp3;
}

//------------------------- Div -------------------------

pub trait DivExp<E: Exponent> {
    type DivExpOutput: Exponent;
}

impl DivExp<Exp_3> for Exp_3 {
    type DivExpOutput = Exp0;
}
impl DivExp<Exp_2> for Exp_3 {
    type DivExpOutput = Exp_1;
}
impl DivExp<Exp_1> for Exp_3 {
    type DivExpOutput = Exp_2;
}
impl DivExp<Exp0> for Exp_3 {
    type DivExpOutput = Exp_3;
}

impl DivExp<Exp_3> for Exp_2 {
    type DivExpOutput = Exp1;
}
impl DivExp<Exp_2> for Exp_2 {
    type DivExpOutput = Exp0;
}
impl DivExp<Exp_1> for Exp_2 {
    type DivExpOutput = Exp_1;
}
impl DivExp<Exp0> for Exp_2 {
    type DivExpOutput = Exp_2;
}
impl DivExp<Exp1> for Exp_2 {
    type DivExpOutput = Exp_3;
}

impl DivExp<Exp_3> for Exp_1 {
    type DivExpOutput = Exp2;
}
impl DivExp<Exp_2> for Exp_1 {
    type DivExpOutput = Exp1;
}
impl DivExp<Exp_1> for Exp_1 {
    type DivExpOutput = Exp0;
}
impl DivExp<Exp0> for Exp_1 {
    type DivExpOutput = Exp_1;
}
impl DivExp<Exp1> for Exp_1 {
    type DivExpOutput = Exp_2;
}
impl DivExp<Exp2> for Exp_1 {
    type DivExpOutput = Exp_3;
}

impl DivExp<Exp_3> for Exp0 {
    type DivExpOutput = Exp3;
}
impl DivExp<Exp_2> for Exp0 {
    type DivExpOutput = Exp2;
}
impl DivExp<Exp_1> for Exp0 {
    type DivExpOutput = Exp1;
}
impl DivExp<Exp0> for Exp0 {
    type DivExpOutput = Exp0;
}
impl DivExp<Exp1> for Exp0 {
    type DivExpOutput = Exp_1;
}
impl DivExp<Exp2> for Exp0 {
    type DivExpOutput = Exp_2;
}
impl DivExp<Exp3> for Exp0 {
    type DivExpOutput = Exp_3;
}

impl DivExp<Exp_2> for Exp1 {
    type DivExpOutput = Exp3;
}
impl DivExp<Exp_1> for Exp1 {
    type DivExpOutput = Exp2;
}
impl DivExp<Exp0> for Exp1 {
    type DivExpOutput = Exp1;
}
impl DivExp<Exp1> for Exp1 {
    type DivExpOutput = Exp0;
}
impl DivExp<Exp2> for Exp1 {
    type DivExpOutput = Exp_1;
}
impl DivExp<Exp3> for Exp1 {
    type DivExpOutput = Exp_2;
}

impl DivExp<Exp_1> for Exp2 {
    type DivExpOutput = Exp3;
}
impl DivExp<Exp0> for Exp2 {
    type DivExpOutput = Exp2;
}
impl DivExp<Exp1> for Exp2 {
    type DivExpOutput = Exp1;
}
impl DivExp<Exp2> for Exp2 {
    type DivExpOutput = Exp0;
}
impl DivExp<Exp3> for Exp2 {
    type DivExpOutput = Exp_1;
}

impl DivExp<Exp0> for Exp3 {
    type DivExpOutput = Exp3;
}
impl DivExp<Exp1> for Exp3 {
    type DivExpOutput = Exp2;
}
impl DivExp<Exp2> for Exp3 {
    type DivExpOutput = Exp1;
}
impl DivExp<Exp3> for Exp3 {
    type DivExpOutput = Exp0;
}

//------------------------- Neg -------------------------

pub trait NegExp {
    type NegOutput: Exponent;
}

impl NegExp for Exp_3 {
    type NegOutput = Exp3;
}
impl NegExp for Exp_2 {
    type NegOutput = Exp2;
}
impl NegExp for Exp_1 {
    type NegOutput = Exp1;
}
impl NegExp for Exp0 {
    type NegOutput = Exp0;
}
impl NegExp for Exp1 {
    type NegOutput = Exp_1;
}
impl NegExp for Exp2 {
    type NegOutput = Exp_2;
}
impl NegExp for Exp3 {
    type NegOutput = Exp_3;
}
