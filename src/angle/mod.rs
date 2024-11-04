pub mod radian;
pub use radian::*;

pub mod degree;
pub use degree::*;

mod imp;

use crate::*;

pub trait AngleOps {
    const PI: Self;

    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn tan(&self) -> Self;

    fn asin(&self) -> Self;
    fn acos(&self) -> Self;
    fn atan(&self) -> Self;
    fn atan2(&self, value: Self) -> Self;

    fn to_degrees(&self) -> Self;
    fn to_radians(&self) -> Self;
}

pub trait Angle<T>: Scalar
where
    T: Scalar + AngleOps,
{
    fn sin(&self) -> T;
    fn cos(&self) -> T;
    fn tan(&self) -> T;

    fn to_degrees(&self) -> Degree<T>;
    fn to_radians(&self) -> Radian<T>;
}

pub trait AngleFactory: Number {
    fn asin(&self) -> Radian<<Self as HasValue>::Output>;
    fn acos(&self) -> Radian<<Self as HasValue>::Output>;
    fn atan(&self) -> Radian<<Self as HasValue>::Output>;
    fn atan2(&self, y: Self) -> Radian<<Self as HasValue>::Output>;
}
