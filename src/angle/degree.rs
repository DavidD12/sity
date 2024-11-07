use crate::*;

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Degree<T: Scalar> {
    value: T,
}

//------------------------- -------------------------

impl<T: Scalar> Degree<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

//------------------------- From/Into -------------------------

impl<T: Scalar> From<T> for Degree<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}

//------------------------- Number -------------------------

impl<T: Scalar> HasValue for Degree<T> {
    type Output = T;

    fn value(self) -> Self::Output {
        self.value
    }
}

//------------------------- Number -------------------------

impl<T: Scalar> Number for Degree<T> {
    const ZERO: Self = Self { value: T::ZERO };
    const ONE: Self = Self { value: T::ONE };
    const EPSILON: Self = Self { value: T::EPSILON };

    fn abs(self) -> Self {
        Self {
            value: self.value.abs(),
        }
    }

    fn min(self, other: Self) -> Self {
        Self {
            value: self.value.min(other.value),
        }
    }

    fn max(self, other: Self) -> Self {
        Self {
            value: self.value.max(other.value),
        }
    }

    fn floor(self) -> Self {
        Self {
            value: self.value.floor(),
        }
    }

    fn round(self) -> Self {
        Self {
            value: self.value.round(),
        }
    }

    fn ceil(self) -> Self {
        Self {
            value: self.value.ceil(),
        }
    }

    fn trunc(self) -> Self {
        Self {
            value: self.value.trunc(),
        }
    }
}

//------------------------- Display -------------------------

impl<T: Scalar> std::fmt::Display for Degree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}°", self.value)
    }
}

impl<T> Angle<T> for Degree<T>
where
    T: Scalar + AngleOps,
{
    fn sin(&self) -> T {
        self.value.sin()
    }

    fn cos(&self) -> T {
        self.value.cos()
    }

    fn tan(&self) -> T {
        self.value.tan()
    }

    fn to_degrees(&self) -> Degree<T> {
        *self
    }

    fn to_radians(&self) -> Radian<T> {
        Radian::new(self.value.to_radians())
    }
}

//------------------------- Neg -------------------------

impl<T: Scalar + std::ops::Neg<Output = T>> std::ops::Neg for Degree<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { value: -self.value }
    }
}

//------------------------- Add -------------------------

impl<T: Scalar> std::ops::Add<Self> for Degree<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl<T: Scalar> std::ops::AddAssign<Self> for Degree<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value
    }
}

//------------------------- Sub -------------------------

impl<T: Scalar> std::ops::Sub<Self> for Degree<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl<T: Scalar> std::ops::SubAssign<Self> for Degree<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value
    }
}

//------------------------- Mul -------------------------

impl<T: Scalar> std::ops::Mul<Self> for Degree<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
        }
    }
}

impl<T: Scalar> std::ops::MulAssign<Self> for Degree<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value
    }
}

//------------------------- Div -------------------------

impl<T: Scalar> std::ops::Div<Self> for Degree<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value / rhs.value,
        }
    }
}

impl<T: Scalar> std::ops::DivAssign<Self> for Degree<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value
    }
}
