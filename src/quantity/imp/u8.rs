use crate::*;
use std::ops::*;

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

//------------------------- Integer -------------------------

impl<LE, ME, TE, IE, OE> Integer for Quantity<u8, LE, ME, TE, IE, OE>
where
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    fn min(self, other: Self) -> Self {
        Self::new(self.value().min(other.value()))
    }

    fn max(self, other: Self) -> Self {
        Self::new(self.value().max(other.value()))
    }
}

//------------------------- Pow2 -------------------------

impl<LE, ME, TE, IE, OE> Pow2 for Quantity<u8, LE, ME, TE, IE, OE>
where
    LE: Exponent + Pow2Exp,
    ME: Exponent + Pow2Exp,
    TE: Exponent + Pow2Exp,
    IE: Exponent + Pow2Exp,
    OE: Exponent + Pow2Exp,
{
    type Output = Quantity<u8, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;
    fn pow2(self) -> Self::Output {
        Self::Output::new(self.value().pow(2))
    }
}

//------------------------- Pow3 -------------------------

impl<LE, ME, TE, IE, OE> Pow3 for Quantity<u8, LE, ME, TE, IE, OE>
where
    LE: Exponent + Pow3Exp,
    ME: Exponent + Pow3Exp,
    TE: Exponent + Pow3Exp,
    IE: Exponent + Pow3Exp,
    OE: Exponent + Pow3Exp,
{
    type Output = Quantity<u8, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;
    fn pow3(self) -> Self::Output {
        Self::Output::new(self.value().pow(3))
    }
}
