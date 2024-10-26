use sealed::sealed;

#[sealed]
pub trait Exponent: Copy {
    const EXPONENT: i32;
}

#[derive(Copy, Clone, Debug)]
pub struct Exp<const N: i32>;

#[sealed]
impl<const N: i32> Exponent for Exp<N> {
    const EXPONENT: i32 = N;
}

pub fn pretty_exponent(exp: i32) -> String {
    match exp {
        1 => "".to_string(),
        2 => "²".to_string(),
        3 => "³".to_string(),
        x => x.to_string(),
    }
}

//------------------------- Mul -------------------------
pub trait MulExp<E: Exponent> {
    type Output: Exponent;
}
//------------------------- Div -------------------------
pub trait DivExp<E: Exponent> {
    type Output: Exponent;
}
//------------------------- Pow -------------------------
pub trait PowExp<const N: i32> {
    type Output: Exponent;
}
//------------------------- Root -------------------------
pub trait RootExp<const N: i32> {
    type Output: Exponent;
}
