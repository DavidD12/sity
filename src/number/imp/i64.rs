use crate::*;
use std::ops::*;

//------------------------- Number -------------------------

impl Number for i64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

//------------------------- Signed -------------------------

impl Signed for i64 {
    fn abs(self) -> Self {
        self.abs()
    }
}

//------------------------- Pow -------------------------

impl Pow2 for i64 {
    type Output = Self;

    fn pow2(self) -> Self::Output {
        self.pow(2)
    }
}

impl Pow3 for i64 {
    type Output = Self;

    fn pow3(self) -> Self::Output {
        self.pow(3)
    }
}

//------------------------- Mul -------------------------

impl<LE, ME, TE, IE, OE> Mul<Quantity<i64, LE, ME, TE, IE, OE>> for i64
where
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    type Output = Quantity<i64, LE, ME, TE, IE, OE>;
    fn mul(self, other: Quantity<i64, LE, ME, TE, IE, OE>) -> Self::Output {
        Self::Output::new(self * other.value())
    }
}
