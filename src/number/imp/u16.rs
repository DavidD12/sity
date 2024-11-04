use crate::*;
use std::ops::*;

//------------------------- HasValue -------------------------

impl HasValue for u16 {
    type Output = Self;

    fn value(self) -> Self::Output {
        self
    }
}
//------------------------- Number -------------------------

impl Number for u16 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const EPSILON: Self = 0;

    fn abs(self) -> Self {
        self
    }

    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }

    fn floor(self) -> Self {
        self
    }

    fn round(self) -> Self {
        self
    }

    fn ceil(self) -> Self {
        self
    }

    fn trunc(self) -> Self {
        self
    }
}

//------------------------- Scalar -------------------------

impl Scalar for u16 {}

//------------------------- Pow -------------------------

impl Pow2 for u16 {
    type Output = Self;

    fn pow2(&self) -> <Self as Pow2>::Output {
        self.pow(2)
    }
}

impl Pow3 for u16 {
    type Output = Self;

    fn pow3(&self) -> <Self as Pow3>::Output {
        self.pow(4)
    }
}

impl Pow4 for u16 {
    type Output = Self;

    fn pow4(&self) -> <Self as Pow3>::Output {
        self.pow(4)
    }
}
//------------------------- Mul -------------------------

impl<LE, ME, TE, IE, OE> Mul<Qt<u16, LE, ME, TE, IE, OE>> for u16
where
    LE: ScaleFactor,
    ME: ScaleFactor,
    TE: ScaleFactor,
    IE: ScaleFactor,
    OE: ScaleFactor,
{
    type Output = Qt<u16, LE, ME, TE, IE, OE>;
    fn mul(self, other: Qt<u16, LE, ME, TE, IE, OE>) -> Self::Output {
        Self::Output::new(self * other.value())
    }
}
