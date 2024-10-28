use crate::*;

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Degree<T: Number> {
    value: T,
}

//------------------------- -------------------------

impl<T: Number> Degree<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

//------------------------- From/Into -------------------------

impl<T: Number> From<T> for Degree<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}

//------------------------- Number -------------------------

impl<T: Number> HasValue for Degree<T> {
    type Output = T;

    fn value(self) -> Self::Output {
        self.value
    }
}

//------------------------- Number -------------------------

impl<T: Number> Number for Degree<T> {
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

impl<T: Number> std::fmt::Display for Degree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}rad", self.value)
    }
}

impl<T> Angle<T> for Degree<T>
where
    T: Number + AngleOps,
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

    // fn asin(value: T) -> Self {
    //     Self {
    //         value: value.asin(),
    //     }
    // }

    // fn acos(value: T) -> Self {
    //     Self {
    //         value: value.acos(),
    //     }
    // }

    // fn atan(value: T) -> Self {
    //     Self {
    //         value: value.atan(),
    //     }
    // }

    // fn atan2(x: T, y: T) -> Self {
    //     Self { value: x.atan2(y) }
    // }

    fn to_degrees(&self) -> Degree<T> {
        *self
    }

    fn to_radians(&self) -> Radian<T> {
        Radian::new(self.value.to_radians())
    }
}

//------------------------- Add -------------------------

impl<T: Number> std::ops::Add<Self> for Degree<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl<T: Number> std::ops::AddAssign<Self> for Degree<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value
    }
}

//------------------------- Sub -------------------------

impl<T: Number> std::ops::Sub<Self> for Degree<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl<T: Number> std::ops::SubAssign<Self> for Degree<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value
    }
}
