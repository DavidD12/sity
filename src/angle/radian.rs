use crate::*;

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Radian<T: Scalar + AngleOps> {
    value: T,
}

//------------------------- -------------------------

impl<T: Scalar + AngleOps> Radian<T> {
    pub const PI: Self = Self { value: T::PI };

    pub fn new(value: T) -> Self {
        Self { value }
    }
}

//------------------------- From/Into -------------------------

impl<T: Scalar + AngleOps> From<T> for Radian<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}

//------------------------- Number -------------------------

impl<T: Scalar + AngleOps> HasValue for Radian<T> {
    type Output = T;

    fn value(self) -> Self::Output {
        self.value
    }
}

//------------------------- Number -------------------------

impl<T: Scalar + AngleOps> Number for Radian<T> {
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

impl<T: Scalar + AngleOps> std::fmt::Display for Radian<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}rad", self.value)
    }
}

impl<T> Angle<T> for Radian<T>
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

    fn to_radians(&self) -> Self {
        *self
    }

    fn to_degrees(&self) -> Degree<T> {
        Degree::new(self.value.to_degrees())
    }
}

//------------------------- Neg -------------------------

impl<T: Scalar + std::ops::Neg<Output = T> + AngleOps> std::ops::Neg for Radian<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { value: -self.value }
    }
}

//------------------------- Add -------------------------

impl<T: Scalar + AngleOps> std::ops::Add<Self> for Radian<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl<T: Scalar + AngleOps> std::ops::AddAssign<Self> for Radian<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value
    }
}

//------------------------- Sub -------------------------

impl<T: Scalar + AngleOps> std::ops::Sub<Self> for Radian<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl<T: Scalar + AngleOps> std::ops::SubAssign<Self> for Radian<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value
    }
}

//------------------------- Mul -------------------------

impl<T: Scalar + AngleOps> std::ops::Mul<Self> for Radian<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
        }
    }
}

impl<T: Scalar + AngleOps> std::ops::MulAssign<Self> for Radian<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value
    }
}

//------------------------- Div -------------------------

impl<T: Scalar + AngleOps> std::ops::Div<Self> for Radian<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value / rhs.value,
        }
    }
}

impl<T: Scalar + AngleOps> std::ops::DivAssign<Self> for Radian<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value
    }
}
