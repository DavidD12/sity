//------------------------- Integer -------------------------

pub trait Integer {
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

//------------------------- Signed -------------------------

pub trait Signed {
    fn abs(self) -> Self;
}

//------------------------- Float -------------------------

pub trait Float {
    fn floor(self) -> Self;
    fn round(self) -> Self;
    fn ceil(self) -> Self;
    fn trunc(self) -> Self;
}

//------------------------- Pow -------------------------

pub trait Pow2 {
    type Output;
    fn pow2(self) -> Self::Output;
}

pub trait Pow3 {
    type Output;
    fn pow3(self) -> Self::Output;
}

//------------------------- Root -------------------------

pub trait Root2 {
    type Output;
    fn root2(self) -> Self::Output;
}

pub trait Root3 {
    type Output;
    fn root3(self) -> Self::Output;
}
