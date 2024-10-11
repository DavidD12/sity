use crate::*;
use std::ops::*;

//------------------------- Number -------------------------

impl Number for isize {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

//------------------------- Signed -------------------------

impl Signed for isize {
    fn abs(self) -> Self {
        self.abs()
    }
}

//------------------------- Pow -------------------------

impl Pow2 for isize {
    type Output = Self;

    fn pow2(self) -> Self::Output {
        self.pow(2)
    }
}

impl Pow3 for isize {
    type Output = Self;

    fn pow3(self) -> Self::Output {
        self.pow(3)
    }
}

//------------------------- Mul -------------------------

impl<LE, ME, TE, IE, OE> Mul<Quantity<isize, LE, ME, TE, IE, OE>> for isize
where
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    type Output = Quantity<isize, LE, ME, TE, IE, OE>;
    fn mul(self, other: Quantity<isize, LE, ME, TE, IE, OE>) -> Self::Output {
        Self::Output::new(self * other.value())
    }
}
