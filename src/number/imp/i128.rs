use crate::*;
use std::ops::*;

//------------------------- HasValue -------------------------

impl HasValue for i128 {
    type Output = Self;

    fn value(self) -> Self::Output {
        self
    }
}

//------------------------- Number -------------------------

impl Number for i128 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const EPSILON: Self = 0;

    fn abs(self) -> Self {
        self.abs()
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

impl Scalar for i128 {}

//------------------------- Pow -------------------------

impl Pow2 for i128 {
    type Output = Self;

    fn pow2(&self) -> <Self as Pow2>::Output {
        self.pow(2)
    }
}

impl Pow3 for i128 {
    type Output = Self;

    fn pow3(&self) -> <Self as Pow3>::Output {
        self.pow(4)
    }
}

impl Pow4 for i128 {
    type Output = Self;

    fn pow4(&self) -> <Self as Pow3>::Output {
        self.pow(4)
    }
}

//------------------------- Mul -------------------------

impl<LE, ME, TE, IE, OE> Mul<Qt<i128, LE, ME, TE, IE, OE>> for i128
where
    LE: ScaleFactor,
    ME: ScaleFactor,
    TE: ScaleFactor,
    IE: ScaleFactor,
    OE: ScaleFactor,
{
    type Output = Qt<i128, LE, ME, TE, IE, OE>;
    fn mul(self, other: Qt<i128, LE, ME, TE, IE, OE>) -> Self::Output {
        Self::Output::new(self * other.value())
    }
}
