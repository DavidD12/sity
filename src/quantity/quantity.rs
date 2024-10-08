use super::*;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::*;

#[derive(Copy, Clone)]
pub struct Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy,
    LE: Exponent, // Length (m)
    ME: Exponent, // Mass (kg)
    TE: Exponent, // Time (s)
    IE: Exponent, // Electric current (A)
    OE: Exponent, // Thermodynamic temperature (K)
{
    value: T,
    le: PhantomData<LE>,
    me: PhantomData<ME>,
    te: PhantomData<TE>,
    ie: PhantomData<IE>,
    oe: PhantomData<OE>,
}

impl<T, LE, ME, TE, IE, OE> Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            le: PhantomData,
            me: PhantomData,
            te: PhantomData,
            ie: PhantomData,
            oe: PhantomData,
        }
    }

    pub fn value(&self) -> T {
        self.value
    }

    pub fn unit(&self) -> String {
        let mut sep = false;
        let mut s = String::new();
        if LE::value() != 0 {
            s += &format!("m{}", LE::pretty());
            sep = true;
        }
        if ME::value() != 0 {
            if sep {
                s += ".";
            }
            s += &format!("kg{}", ME::pretty());
            sep = true;
        }
        if TE::value() != 0 {
            if sep {
                s += ".";
            }
            s += &format!("s{}", TE::pretty());
            sep = true;
        }
        if IE::value() != 0 {
            if sep {
                s += ".";
            }
            s += &format!("A{}", IE::pretty());
            sep = true;
        }
        if OE::value() != 0 {
            if sep {
                s += ".";
            }
            s += &format!("K{}", OE::pretty());
            // sep = true;
        }
        s
    }
}

impl<T, LE, ME, TE, IE, OE> Debug for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + Display,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value().fmt(f)?;
        write!(f, "{}", self.unit())
    }
}

impl<T, LE, ME, TE, IE, OE> From<T> for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + Display,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    fn from(value: T) -> Self {
        Self::new(value.into())
    }
}

//------------------------- Add -------------------------

impl<T, LE, ME, TE, IE, OE> Add<Self> for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + Add<T, Output = T>,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl<T, LE, ME, TE, IE, OE> AddAssign<Self> for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + AddAssign<T>,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    fn add_assign(&mut self, other: Self) {
        self.value += other.value
    }
}

//------------------------- Sub -------------------------

impl<T, LE, ME, TE, IE, OE> Sub<Self> for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + Sub<T, Output = T>,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

impl<T, LE, ME, TE, IE, OE> SubAssign<Self> for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + SubAssign<T>,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value
    }
}

//------------------------- Mul -------------------------

impl<T, LE, ME, TE, IE, OE> Mul<T> for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + Mul<T, Output = T>,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self::new(self.value * other)
    }
}

impl<T, LE, ME, TE, IE, OE> MulAssign<T> for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + MulAssign<T>,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    fn mul_assign(&mut self, other: T) {
        self.value *= other
    }
}

impl<T, LE, ME, TE, IE, OE, LE1, ME1, TE1, IE1, OE1> Mul<Quantity<T, LE1, ME1, TE1, IE1, OE1>>
    for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + Mul<T, Output = T>,
    LE: Exponent + MulExp<LE1>,
    ME: Exponent + MulExp<ME1>,
    TE: Exponent + MulExp<TE1>,
    IE: Exponent + MulExp<IE1>,
    OE: Exponent + MulExp<OE1>,
    LE1: Exponent,
    ME1: Exponent,
    TE1: Exponent,
    IE1: Exponent,
    OE1: Exponent,
{
    type Output = Quantity<T, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;

    fn mul(self, other: Quantity<T, LE1, ME1, TE1, IE1, OE1>) -> Self::Output {
        Self::Output::new(self.value * other.value)
    }
}

//------------------------- Div -------------------------

impl<T, LE, ME, TE, IE, OE> Div<T> for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + Div<T, Output = T>,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self::new(self.value / other)
    }
}

impl<T, LE, ME, TE, IE, OE> DivAssign<T> for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + DivAssign<T>,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    fn div_assign(&mut self, other: T) {
        self.value /= other
    }
}

impl<T, LE, ME, TE, IE, OE, LE1, ME1, TE1, IE1, OE1> Div<Quantity<T, LE1, ME1, TE1, IE1, OE1>>
    for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + Div<T, Output = T>,
    LE: Exponent + DivExp<LE1>,
    ME: Exponent + DivExp<ME1>,
    TE: Exponent + DivExp<TE1>,
    IE: Exponent + DivExp<IE1>,
    OE: Exponent + DivExp<OE1>,
    LE1: Exponent,
    ME1: Exponent,
    TE1: Exponent,
    IE1: Exponent,
    OE1: Exponent,
{
    type Output = Quantity<
        T,
        LE::DivExpOutput,
        ME::DivExpOutput,
        TE::DivExpOutput,
        IE::DivExpOutput,
        OE::DivExpOutput,
    >;

    fn div(self, other: Quantity<T, LE1, ME1, TE1, IE1, OE1>) -> Self::Output {
        Self::Output::new(self.value / other.value)
    }
}

//------------------------- Inverse -------------------------

impl<T, LE, ME, TE, IE, OE> Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy,
    LE: Exponent + NegExp,
    ME: Exponent + NegExp,
    TE: Exponent + NegExp,
    IE: Exponent + NegExp,
    OE: Exponent + NegExp,
{
    pub fn inverse(
        &self,
    ) -> Quantity<T, LE::NegOutput, ME::NegOutput, TE::NegOutput, IE::NegOutput, OE::NegOutput>
    {
        Quantity::<T, LE::NegOutput, ME::NegOutput, TE::NegOutput, IE::NegOutput, OE::NegOutput>::new(self.value)
    }
}

//------------------------- PartialEq -------------------------

impl<T, LE, ME, TE, IE, OE> PartialEq for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + PartialEq,
    LE: Exponent + NegExp,
    ME: Exponent + NegExp,
    TE: Exponent + NegExp,
    IE: Exponent + NegExp,
    OE: Exponent + NegExp,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

//------------------------- PartialOrd -------------------------

impl<T, LE, ME, TE, IE, OE> PartialOrd for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Copy + PartialOrd,
    LE: Exponent + NegExp,
    ME: Exponent + NegExp,
    TE: Exponent + NegExp,
    IE: Exponent + NegExp,
    OE: Exponent + NegExp,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value())
    }
}
