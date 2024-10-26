use crate::*;
use std::ops::*;

//------------------------- Number -------------------------

impl Number for i128 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

//------------------------- Signed -------------------------

impl Signed for i128 {
    fn abs(self) -> Self {
        self.abs()
    }
}

//------------------------- Pow -------------------------

impl Pow2 for i128 {
    type Output = Self;

    fn pow2(self) -> Self::Output {
        self.pow(2)
    }
}

impl Pow3 for i128 {
    type Output = Self;

    fn pow3(self) -> Self::Output {
        self.pow(3)
    }
}

//------------------------- Mul -------------------------

impl<LE, ME, TE, IE, OE> Mul<Quantity<i128, LE, ME, TE, IE, OE>> for i128
where
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    type Output = Quantity<i128, LE, ME, TE, IE, OE>;
    fn mul(self, other: Quantity<i128, LE, ME, TE, IE, OE>) -> Self::Output {
        Self::Output::new(self * other.value())
    }
}
