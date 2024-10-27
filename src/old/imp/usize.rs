use crate::*;
use std::ops::*;

//------------------------- HasValue -------------------------

impl HasValue for usize {
    type Output = Self;

    fn value(self) -> Self::Output {
        self
    }
}
//------------------------- Number -------------------------

impl Number for usize {
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

//------------------------- Pow -------------------------

impl Pow2 for usize {
    type Output = Self;

    fn pow2(self) -> <Self as Pow2>::Output {
        self.pow(2)
    }
}

impl Pow3 for usize {
    type Output = Self;

    fn pow3(self) -> <Self as Pow3>::Output {
        self.pow(3)
    }
}

//------------------------- Mul -------------------------

impl<LE, ME, TE, IE, OE> Mul<Quantity<usize, LE, ME, TE, IE, OE>> for usize
where
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    type Output = Quantity<usize, LE, ME, TE, IE, OE>;
    fn mul(self, other: Quantity<usize, LE, ME, TE, IE, OE>) -> Self::Output {
        Self::Output::new(self * other.value())
    }
}