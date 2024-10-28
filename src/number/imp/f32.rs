use crate::*;

//------------------------- HasValue -------------------------

impl HasValue for f32 {
    type Output = Self;

    fn value(self) -> Self::Output {
        self
    }
}

//------------------------- ToBase -------------------------

impl ToBase for f32 {
    fn to_base<P1: Prefix, P2: Prefix, const N: i32>(&self) -> Self {
        self * 10.0_f32.powi(P1::PREFIX - P2::PREFIX).powi(N)
    }
}

//------------------------- Number -------------------------

impl Number for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const EPSILON: Self = std::f32::EPSILON;

    fn abs(self) -> Self {
        self.abs()
    }

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

impl Pow2 for f32 {
    type Output = Self;

    fn pow2(&self) -> <Self as Pow2>::Output {
        self.powi(2)
    }
}

impl Pow3 for f32 {
    type Output = Self;

    fn pow3(&self) -> <Self as Pow3>::Output {
        self.powi(2)
    }
}

impl Root2 for f32 {
    type Output = Self;

    fn root2(&self) -> <Self as Root2>::Output {
        self.sqrt()
    }
}

impl Root3 for f32 {
    type Output = Self;

    fn root3(&self) -> <Self as Root3>::Output {
        self.cbrt()
    }
}

//------------------------- Mul -------------------------

impl<LE, ME, TE, IE, OE> std::ops::Mul<Qt<f32, LE, ME, TE, IE, OE>> for f32
where
    LE: ScaleFactor,
    ME: ScaleFactor,
    TE: ScaleFactor,
    IE: ScaleFactor,
    OE: ScaleFactor,
{
    type Output = Qt<f32, LE, ME, TE, IE, OE>;
    fn mul(self, other: Qt<f32, LE, ME, TE, IE, OE>) -> Self::Output {
        Self::Output::new(self * other.value())
    }
}

//------------------------- Div -------------------------

// TODO !

// impl<LE, ME, TE, IE, OE> std::ops::Div<Qt<f32, LE, ME, TE, IE, OE>> for f32
// where
//     LE: ScaleFactor,
//     ME: ScaleFactor,
//     TE: ScaleFactor,
//     IE: ScaleFactor,
//     OE: ScaleFactor,
// {
//     type Output = Qt<f32, LE, ME, TE, IE, OE>;
//     fn mul(self, other: Qt<f32, LE, ME, TE, IE, OE>) -> Self::Output {
//         self * other.inverse()
//     }
// }
