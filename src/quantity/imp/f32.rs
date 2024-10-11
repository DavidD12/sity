use crate::*;
use std::ops::*;

//------------------------- Pow/Root -------------------------

impl Pow2 for f32 {
    type Output = Self;

    fn pow2(self) -> Self::Output {
        self.powi(2)
    }
}

impl Pow3 for f32 {
    type Output = Self;

    fn pow3(self) -> Self::Output {
        self.powi(2)
    }
}

impl Root2 for f32 {
    type Output = Self;

    fn root2(self) -> Self::Output {
        self.sqrt()
    }
}

impl Root3 for f32 {
    type Output = Self;

    fn root3(self) -> Self::Output {
        self.cbrt()
    }
}

//------------------------- Mul -------------------------

impl<LE, ME, TE, IE, OE> Mul<Quantity<f32, LE, ME, TE, IE, OE>> for f32
where
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    type Output = Quantity<f32, LE, ME, TE, IE, OE>;
    fn mul(self, other: Quantity<f32, LE, ME, TE, IE, OE>) -> Self::Output {
        Self::Output::new(self * other.value())
    }
}

//------------------------- Integer -------------------------

impl<LE, ME, TE, IE, OE> Integer for Quantity<f32, LE, ME, TE, IE, OE>
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

impl<LE, ME, TE, IE, OE> Signed for Quantity<f32, LE, ME, TE, IE, OE>
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

impl<LE, ME, TE, IE, OE> Float for Quantity<f32, LE, ME, TE, IE, OE>
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
