use crate::*;
use std::ops::*;

//------------------------- Number -------------------------

impl Number for u8 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

//------------------------- Pow -------------------------

impl Pow2 for u8 {
    type Output = Self;

    fn pow2(self) -> Self::Output {
        self.pow(2)
    }
}

impl Pow3 for u8 {
    type Output = Self;

    fn pow3(self) -> Self::Output {
        self.pow(3)
    }
}

//------------------------- Mul -------------------------

impl<LE, ME, TE, IE, OE> Mul<Quantity<u8, LE, ME, TE, IE, OE>> for u8
where
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    type Output = Quantity<u8, LE, ME, TE, IE, OE>;
    fn mul(self, other: Quantity<u8, LE, ME, TE, IE, OE>) -> Self::Output {
        Self::Output::new(self * other.value())
    }
}
