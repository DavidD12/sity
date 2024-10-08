use super::*;
use std::fmt::Display;

pub trait Derived {
    fn symbol() -> &'static str;
}

impl<T, LE, ME, TE, IE, OE> Display for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + Display,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
    Self: Derived,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value().fmt(f)?;
        write!(f, "{}", Self::symbol())
    }
}
