use crate::*;
use std::ops::*;

//------------------------- Mul -------------------------

impl<LE, ME, TE, IE, OE> Mul<Quantity<f64, LE, ME, TE, IE, OE>> for f64
where
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    type Output = Quantity<f64, LE, ME, TE, IE, OE>;
    fn mul(self, other: Quantity<f64, LE, ME, TE, IE, OE>) -> Self::Output {
        Self::Output::new(self * other.value())
    }
}

//------------------------- Integer -------------------------

impl<LE, ME, TE, IE, OE> Integer for Quantity<f64, LE, ME, TE, IE, OE>
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

//------------------------- Signed -------------------------

impl<LE, ME, TE, IE, OE> Signed for Quantity<f64, LE, ME, TE, IE, OE>
where
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    fn abs(self) -> Self {
        Self::new(self.value().abs())
    }
}

//------------------------- Float -------------------------

impl<LE, ME, TE, IE, OE> Float for Quantity<f64, LE, ME, TE, IE, OE>
where
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    fn floor(self) -> Self {
        Self::new(self.value().floor())
    }

    fn round(self) -> Self {
        Self::new(self.value().round())
    }

    fn ceil(self) -> Self {
        Self::new(self.value().ceil())
    }

    fn trunc(self) -> Self {
        Self::new(self.value().trunc())
    }
}

//------------------------- Pow2 -------------------------

impl<LE, ME, TE, IE, OE> Pow2 for Quantity<f64, LE, ME, TE, IE, OE>
where
    LE: Exponent + Pow2Exp,
    ME: Exponent + Pow2Exp,
    TE: Exponent + Pow2Exp,
    IE: Exponent + Pow2Exp,
    OE: Exponent + Pow2Exp,
{
    type Output = Quantity<f64, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;
    fn pow2(self) -> Self::Output {
        Self::Output::new(self.value().powi(2))
    }
}

//------------------------- Pow3 -------------------------

impl<LE, ME, TE, IE, OE> Pow3 for Quantity<f64, LE, ME, TE, IE, OE>
where
    LE: Exponent + Pow3Exp,
    ME: Exponent + Pow3Exp,
    TE: Exponent + Pow3Exp,
    IE: Exponent + Pow3Exp,
    OE: Exponent + Pow3Exp,
{
    type Output = Quantity<f64, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;
    fn pow3(self) -> Self::Output {
        Self::Output::new(self.value().powi(3))
    }
}

//------------------------- Root2 -------------------------

impl<LE, ME, TE, IE, OE> Root2 for Quantity<f64, LE, ME, TE, IE, OE>
where
    LE: Exponent + Root2Exp,
    ME: Exponent + Root2Exp,
    TE: Exponent + Root2Exp,
    IE: Exponent + Root2Exp,
    OE: Exponent + Root2Exp,
{
    type Output = Quantity<f64, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;
    fn sqrt(self) -> Self::Output {
        Self::Output::new(self.value().sqrt())
    }
}

//------------------------- Root3 -------------------------

impl<LE, ME, TE, IE, OE> Root3 for Quantity<f64, LE, ME, TE, IE, OE>
where
    LE: Exponent + Root3Exp,
    ME: Exponent + Root3Exp,
    TE: Exponent + Root3Exp,
    IE: Exponent + Root3Exp,
    OE: Exponent + Root3Exp,
{
    type Output = Quantity<f64, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;
    fn cbrt(self) -> Self::Output {
        Self::Output::new(self.value().cbrt())
    }
}
