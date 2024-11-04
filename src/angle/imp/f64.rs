use crate::{AngleFactory, AngleOps, Radian};

impl AngleOps for f64 {
    const PI: Self = std::f64::consts::PI;

    fn sin(&self) -> Self {
        (*self).sin()
    }

    fn cos(&self) -> Self {
        (*self).cos()
    }

    fn tan(&self) -> Self {
        (*self).tan()
    }

    fn asin(&self) -> Self {
        (*self).asin()
    }

    fn acos(&self) -> Self {
        (*self).acos()
    }

    fn atan(&self) -> Self {
        (*self).atan()
    }

    fn atan2(&self, value: Self) -> Self {
        (*self).atan2(value)
    }

    fn to_degrees(&self) -> Self {
        (*self).to_degrees()
    }

    fn to_radians(&self) -> Self {
        (*self).to_radians()
    }
}

impl AngleFactory for f64 {
    fn asin(&self) -> Radian<Self> {
        Radian::new((*self).asin())
    }

    fn acos(&self) -> Radian<Self> {
        Radian::new((*self).acos())
    }

    fn atan(&self) -> Radian<Self> {
        Radian::new((*self).atan())
    }

    fn atan2(&self, x: Self) -> Radian<Self> {
        Radian::new((*self).atan2(x))
    }
}
