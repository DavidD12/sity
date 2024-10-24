use super::*;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::*;

pub trait QuantityValue<T: Number>: Number {
    fn value(self) -> T;
}

#[derive(Copy, Clone, Debug)]
pub struct Quantity<T, LE, ME, TE, IE, OE>
where
    T: Number,
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
    T: Number,
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

    pub fn pretty_unit<E: Exponent>(unit: &str) -> String {
        let exp = match E::VALUE {
            1 => "".to_string(),
            2 => "²".to_string(),
            3 => "³".to_string(),
            x => x.to_string(),
        };
        format!("{}{}{}", E::BASE_SYMBOL, unit, exp)
    }

    pub fn unit(&self) -> String {
        let mut sep = false;
        let mut s = String::new();
        // Length (m)
        if LE::VALUE != 0 {
            s += &Self::pretty_unit::<LE>("m");
            sep = true;
        }
        // Mass (Kg)
        if ME::VALUE != 0 {
            if sep {
                s += ".";
            }
            s += &Self::pretty_unit::<ME>("g");
            sep = true;
        }
        // Time (s)
        if TE::VALUE != 0 {
            if sep {
                s += ".";
            }
            s += &Self::pretty_unit::<TE>("s");
            sep = true;
        }
        // Electric (A)
        if IE::VALUE != 0 {
            if sep {
                s += ".";
            }
            s += &Self::pretty_unit::<IE>("s");
            sep = true;
        }
        // Temperature (K)
        if OE::VALUE != 0 {
            if sep {
                s += ".";
            }
            s += &Self::pretty_unit::<OE>("K");
            // sep = true;
        }
        s
    }
}

//------------------------- Default -------------------------

impl<T, LE, ME, TE, IE, OE> Default for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Number,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    fn default() -> Self {
        Self {
            value: T::ZERO,
            le: PhantomData,
            me: PhantomData,
            te: PhantomData,
            ie: PhantomData,
            oe: PhantomData,
        }
    }
}

//------------------------- Convert -------------------------

// pub trait Convert<P: Prefix, Output: Number> {
//     fn convert<PO: Prefix>(&self) -> Output;
// }

// impl<T, LE, ME, TE, IE, OE> Convert< for Quantity<T, Exp1<P>, Exp0, Exp0, Exp0, Exp0>
// where
//     T: Number,
//     P: Prefix,
// {
// }

//------------------------- Display -------------------------

impl<T, LE, ME, TE, IE, OE> Display for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Number + Display,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)?;
        write!(f, "{}", self.unit())
    }
}

//------------------------- From/Into -------------------------

impl<T, LE, ME, TE, IE, OE> From<T> for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Number + Display,
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

//------------------------- HasValue -------------------------

impl<T, LE, ME, TE, IE, OE> HasValue for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Number,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    type Output = T;

    fn value(self) -> Self::Output {
        self.value
    }
}

//------------------------- Number -------------------------

impl<T, LE, ME, TE, IE, OE> Number for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Number,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    const ZERO: Self = Self {
        value: T::ZERO,
        le: PhantomData,
        me: PhantomData,
        te: PhantomData,
        ie: PhantomData,
        oe: PhantomData,
    };

    const ONE: Self = Self {
        value: T::ONE,
        le: PhantomData,
        me: PhantomData,
        te: PhantomData,
        ie: PhantomData,
        oe: PhantomData,
    };

    const EPSILON: Self = Self {
        value: T::EPSILON,
        le: PhantomData,
        me: PhantomData,
        te: PhantomData,
        ie: PhantomData,
        oe: PhantomData,
    };

    fn abs(self) -> Self {
        Self::new(self.value.abs())
    }

    fn min(self, other: Self) -> Self {
        Self::new(self.value.min(other.value))
    }

    fn max(self, other: Self) -> Self {
        Self::new(self.value.max(other.value))
    }

    fn floor(self) -> Self {
        Self::new(self.value.floor())
    }

    fn round(self) -> Self {
        Self::new(self.value.round())
    }

    fn ceil(self) -> Self {
        Self::new(self.value.ceil())
    }

    fn trunc(self) -> Self {
        Self::new(self.value.trunc())
    }
}

//------------------------- PartialEq/PartialOrd -------------------------

impl<T, LE, ME, TE, IE, OE> PartialEq for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Number,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T, LE, ME, TE, IE, OE> PartialOrd for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Number,
    LE: Exponent,
    ME: Exponent,
    TE: Exponent,
    IE: Exponent,
    OE: Exponent,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

//------------------------- Add -------------------------

impl<T, LE, ME, TE, IE, OE> Add<Self> for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Number + Add<T, Output = T>,
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
    T: Number + AddAssign<T>,
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
    T: Number + Sub<T, Output = T>,
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
    T: Number + SubAssign<T>,
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
    T: Number + Mul<T, Output = T>,
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
    T: Number + MulAssign<T>,
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
    T: Number + Mul<T, Output = T>,
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
    T: Number + Div<T, Output = T>,
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
    T: Number + DivAssign<T>,
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
    T: Number + Div<T, Output = T>,
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
    T: Number,
    LE: Exponent + InvExp,
    ME: Exponent + InvExp,
    TE: Exponent + InvExp,
    IE: Exponent + InvExp,
    OE: Exponent + InvExp,
{
    pub fn inverse(
        &self,
    ) -> Quantity<T, LE::NegOutput, ME::NegOutput, TE::NegOutput, IE::NegOutput, OE::NegOutput>
    {
        Quantity::<T, LE::NegOutput, ME::NegOutput, TE::NegOutput, IE::NegOutput, OE::NegOutput>::new(self.value)
    }
}

//------------------------- Pow2 -------------------------

impl<T, LE, ME, TE, IE, OE> Pow2 for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Pow2<Output = T>,
    LE: Exponent + Pow2Exp,
    ME: Exponent + Pow2Exp,
    TE: Exponent + Pow2Exp,
    IE: Exponent + Pow2Exp,
    OE: Exponent + Pow2Exp,
{
    type Output = Quantity<T, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;
    fn pow2(self) -> <Self as Pow2>::Output {
        <Self as Pow2>::Output::new(self.value().pow2())
    }
}

//------------------------- Pow3 -------------------------

impl<T, LE, ME, TE, IE, OE> Pow3 for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Pow3<Output = T>,
    LE: Exponent + Pow3Exp,
    ME: Exponent + Pow3Exp,
    TE: Exponent + Pow3Exp,
    IE: Exponent + Pow3Exp,
    OE: Exponent + Pow3Exp,
{
    type Output = Quantity<T, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;
    fn pow3(self) -> <Self as Pow3>::Output {
        <Self as Pow3>::Output::new(self.value().pow3())
    }
}

//------------------------- Root2 -------------------------

impl<T, LE, ME, TE, IE, OE> Root2 for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Root2<Output = T>,
    LE: Exponent + Root2Exp,
    ME: Exponent + Root2Exp,
    TE: Exponent + Root2Exp,
    IE: Exponent + Root2Exp,
    OE: Exponent + Root2Exp,
{
    type Output = Quantity<T, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;
    fn root2(self) -> <Self as Root2>::Output {
        <Self as Root2>::Output::new(self.value().root2())
    }
}

//------------------------- Root3 -------------------------

impl<T, LE, ME, TE, IE, OE> Root3 for Quantity<T, LE, ME, TE, IE, OE>
where
    T: Root3<Output = T>,
    LE: Exponent + Root3Exp,
    ME: Exponent + Root3Exp,
    TE: Exponent + Root3Exp,
    IE: Exponent + Root3Exp,
    OE: Exponent + Root3Exp,
{
    type Output = Quantity<T, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;
    fn root3(self) -> <Self as Root3>::Output {
        <Self as Root3>::Output::new(self.value().root3())
    }
}
