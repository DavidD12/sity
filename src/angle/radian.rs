use crate::*;

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Radian<T: Number> {
    value: T,
}

//------------------------- -------------------------

impl<T: Number> Radian<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

//------------------------- From/Into -------------------------

impl<T: Number> From<T> for Radian<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}

//------------------------- Number -------------------------

impl<T: Number> HasValue for Radian<T> {
    type Output = T;

    fn value(self) -> Self::Output {
        self.value
    }
}

//------------------------- Number -------------------------

impl<T: Number> Number for Radian<T> {
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

impl<T: Number> std::fmt::Display for Radian<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}rad", self.value)
    }
}

impl<T> Angle<T> for Radian<T>
where
    T: Number + AngleOps,
    <T as HasValue>::Output: AngleOps,
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

    // fn asin(x: T) -> <T as HasValue>::Output {
    //     Self {
    //         value: x.value().asin(),
    //     }
    // }

    // fn acos(value: T) -> <T as HasValue>::Output {
    //     Self {
    //         value: value.acos(),
    //     }
    // }

    // fn atan(value: T) -> <T as HasValue>::Output {
    //     Self {
    //         value: value.atan(),
    //     }
    // }

    // fn atan2(x: T, y: T) -> Self {
    //     Self { value: x.atan2(y) }
    // }

    fn to_radians(&self) -> Self {
        *self
    }

    fn to_degrees(&self) -> Degree<T> {
        Degree::new(self.value.to_degrees())
    }
}

//------------------------- Add -------------------------

impl<T: Number> std::ops::Add<Self> for Radian<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl<T: Number> std::ops::AddAssign<Self> for Radian<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value
    }
}

//------------------------- Sub -------------------------

impl<T: Number> std::ops::Sub<Self> for Radian<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl<T: Number> std::ops::SubAssign<Self> for Radian<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value
    }
}
