//------------------------- Number -------------------------

pub trait Number {
    const ZERO: Self;
    const ONE: Self;
}

//------------------------- Signed -------------------------

pub trait Signed: Number {
    fn abs(self) -> Self;
}

//------------------------- Integer -------------------------

pub trait Integer: Ord {}

//------------------------- Float -------------------------

pub trait Float: Number + Signed {
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn floor(self) -> Self;
    fn round(self) -> Self;
    fn ceil(self) -> Self;
    fn trunc(self) -> Self;
}

//------------------------- Pow -------------------------

pub trait Pow2: Number {
    type Output;
    fn pow2(self) -> Self::Output;
}

pub trait Pow3: Number {
    type Output;
    fn pow3(self) -> Self::Output;
}

//------------------------- Root -------------------------

pub trait Root2: Number {
    type Output;
    fn root2(self) -> Self::Output;
}

pub trait Root3: Number {
    type Output;
    fn root3(self) -> Self::Output;
}
