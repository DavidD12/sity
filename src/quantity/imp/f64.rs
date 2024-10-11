use crate::*;
use std::ops::*;

//------------------------- Number -------------------------

impl Number for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

//------------------------- Signed -------------------------

impl Signed for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
}

//------------------------- Float -------------------------

impl Float for f64 {
    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn floor(self) -> Self {
        self.floor()
    }

    fn round(self) -> Self {
        self.round()
    }

    fn ceil(self) -> Self {
        self.ceil()
    }

    fn trunc(self) -> Self {
        self.trunc()
    }
}

//------------------------- Pow/Root -------------------------

impl Pow2 for f64 {
    type Output = Self;

    fn pow2(self) -> Self::Output {
        self.powi(2)
    }
}

impl Pow3 for f64 {
    type Output = Self;

    fn pow3(self) -> Self::Output {
        self.powi(2)
    }
}

impl Root2 for f64 {
    type Output = Self;

    fn root2(self) -> Self::Output {
        self.sqrt()
    }
}

impl Root3 for f64 {
    type Output = Self;

    fn root3(self) -> Self::Output {
        self.cbrt()
    }
}

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
