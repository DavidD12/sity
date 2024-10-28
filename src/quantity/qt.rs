use super::*;
use crate::angle::*;
use crate::number::*;

use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    value: T,
    length: PhantomData<Length>,
    mass: PhantomData<Mass>,
    time: PhantomData<Time>,
    current: PhantomData<Current>,
    temperature: PhantomData<Temperature>,
}

impl<T, M, G, S, A, K> Quantity for Qt<T, M, G, S, A, K>
where
    T: Number,
    // T: Number,
    M: ScaleFactor,
    G: ScaleFactor,
    S: ScaleFactor,
    A: ScaleFactor,
    K: ScaleFactor,
{
    type Value = T;
    type Length = M;
    type Mass = G;
    type Time = S;
    type Current = A;
    type Temperature = K;
}

impl<T, Length, Mass, Time, Current, Temperature> Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            length: PhantomData,
            mass: PhantomData,
            time: PhantomData,
            current: PhantomData,
            temperature: PhantomData,
        }
    }
}

//------------------------- Default -------------------------

impl<T, Length, Mass, Time, Current, Temperature> Default
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    fn default() -> Self {
        Self {
            value: T::ZERO,
            length: PhantomData,
            mass: PhantomData,
            time: PhantomData,
            current: PhantomData,
            temperature: PhantomData,
        }
    }
}

//------------------------- Display -------------------------

impl<T, Length, Mass, Time, Current, Temperature> std::fmt::Display
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)?;
        write!(f, "{}", self.unit())
    }
}

//------------------------- From/Into -------------------------

impl<T, Length, Mass, Time, Current, Temperature> From<T>
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    fn from(value: T) -> Self {
        Self::new(value.into())
    }
}

//------------------------- Convert -------------------------

impl<
        T,
        MP,
        const ME: i32,
        GP,
        const GE: i32,
        SP,
        const SE: i32,
        AP,
        const AE: i32,
        KP,
        const KE: i32,
    > Qt<T, Scale<MP, ME>, Scale<GP, GE>, Scale<SP, SE>, Scale<AP, AE>, Scale<KP, KE>>
where
    T: Number + ToBase,
    MP: Prefix,
    GP: Prefix,
    SP: Prefix,
    AP: Prefix,
    KP: Prefix,
{
    pub fn convert<MPO, GPO, SPO, APO, KPO>(
        &self,
    ) -> Qt<T, Scale<MPO, ME>, Scale<GPO, GE>, Scale<SPO, SE>, Scale<APO, AE>, Scale<KPO, KE>>
    where
        MPO: Prefix,
        GPO: Prefix,
        SPO: Prefix,
        APO: Prefix,
        KPO: Prefix,
    {
        let v = self.value;
        let v = v.to_base::<MP, MPO, ME>();
        let v = v.to_base::<GP, GPO, GE>();
        let v = v.to_base::<SP, SPO, SE>();
        let v = v.to_base::<AP, APO, AE>();
        let v = v.to_base::<KP, KPO, KE>();
        Qt::<T, Scale<MPO, ME>, Scale<GPO, GE>, Scale<SPO, SE>, Scale<APO, AE>, Scale<KPO, KE>>::new(
            v,
        )
    }
}
//------------------------- HasValue -------------------------

impl<T, Length, Mass, Time, Current, Temperature> HasValue
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    type Output = T;

    fn value(self) -> Self::Output {
        self.value
    }
}

//------------------------- Number -------------------------

impl<T, Length, Mass, Time, Current, Temperature> Number
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    const ZERO: Self = Self {
        value: T::ZERO,
        length: PhantomData,
        mass: PhantomData,
        time: PhantomData,
        current: PhantomData,
        temperature: PhantomData,
    };

    const ONE: Self = Self {
        value: T::ONE,
        length: PhantomData,
        mass: PhantomData,
        time: PhantomData,
        current: PhantomData,
        temperature: PhantomData,
    };

    const EPSILON: Self = Self {
        value: T::EPSILON,
        length: PhantomData,
        mass: PhantomData,
        time: PhantomData,
        current: PhantomData,
        temperature: PhantomData,
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

//------------------------- AngleFactory -------------------------

impl<T, Length, Mass, Time, Current, Temperature> AngleFactory
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number + AngleFactory,
    T: HasValue<Output = T>,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    fn asin(&self) -> Radian<<Self as HasValue>::Output> {
        self.value.asin()
    }

    fn acos(&self) -> Radian<<Self as HasValue>::Output> {
        self.value.acos()
    }

    fn atan(&self) -> Radian<<Self as HasValue>::Output> {
        self.value.atan()
    }

    fn atan2(&self, y: Self) -> Radian<<Self as HasValue>::Output> {
        self.value.atan2(y.value)
    }
}

//------------------------- PartialEq/PartialOrd -------------------------

impl<T, Length, Mass, Time, Current, Temperature> PartialEq
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T, Length, Mass, Time, Current, Temperature> PartialOrd
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

//------------------------- Add -------------------------

impl<T, Length, Mass, Time, Current, Temperature> std::ops::Add<Self>
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl<T, Length, Mass, Time, Current, Temperature> std::ops::AddAssign<Self>
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    fn add_assign(&mut self, other: Self) {
        self.value += other.value
    }
}

//------------------------- Sub -------------------------

impl<T, Length, Mass, Time, Current, Temperature> std::ops::Sub<Self>
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

impl<T, Length, Mass, Time, Current, Temperature> std::ops::SubAssign<Self>
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value
    }
}

//------------------------- Mul -------------------------

impl<T, Length, Mass, Time, Current, Temperature> std::ops::Mul<T>
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number + std::ops::Mul<T, Output = T>,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self::new(self.value * other)
    }
}

impl<T, Length, Mass, Time, Current, Temperature> std::ops::MulAssign<T>
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number + std::ops::MulAssign<T>,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    fn mul_assign(&mut self, other: T) {
        self.value *= other
    }
}

impl<T, LE, ME, TE, IE, OE, LE1, ME1, TE1, IE1, OE1> std::ops::Mul<Qt<T, LE1, ME1, TE1, IE1, OE1>>
    for Qt<T, LE, ME, TE, IE, OE>
where
    T: Number + std::ops::Mul<T, Output = T>,
    LE: ScaleFactor + MulScale<LE1>,
    ME: ScaleFactor + MulScale<ME1>,
    TE: ScaleFactor + MulScale<TE1>,
    IE: ScaleFactor + MulScale<IE1>,
    OE: ScaleFactor + MulScale<OE1>,
    LE1: ScaleFactor,
    ME1: ScaleFactor,
    TE1: ScaleFactor,
    IE1: ScaleFactor,
    OE1: ScaleFactor,
{
    type Output = Qt<T, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;

    fn mul(self, other: Qt<T, LE1, ME1, TE1, IE1, OE1>) -> Self::Output {
        Self::Output::new(self.value * other.value)
    }
}

//------------------------- Div -------------------------

impl<T, Length, Mass, Time, Current, Temperature> std::ops::Div<T>
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number + std::ops::Div<T, Output = T>,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self::new(self.value / other)
    }
}

impl<T, Length, Mass, Time, Current, Temperature> std::ops::DivAssign<T>
    for Qt<T, Length, Mass, Time, Current, Temperature>
where
    T: Number + std::ops::DivAssign<T>,
    Length: ScaleFactor,      // Length (m)
    Mass: ScaleFactor,        // Mass (kg)
    Time: ScaleFactor,        // Time (s)
    Current: ScaleFactor,     // Electric current (A)
    Temperature: ScaleFactor, // Thermodynamic temperature (K)
{
    fn div_assign(&mut self, other: T) {
        self.value /= other
    }
}

impl<T, LE, ME, TE, IE, OE, LE1, ME1, TE1, IE1, OE1> std::ops::Div<Qt<T, LE1, ME1, TE1, IE1, OE1>>
    for Qt<T, LE, ME, TE, IE, OE>
where
    T: Number + std::ops::Div<T, Output = T>,
    LE: ScaleFactor + DivScale<LE1>,
    ME: ScaleFactor + DivScale<ME1>,
    TE: ScaleFactor + DivScale<TE1>,
    IE: ScaleFactor + DivScale<IE1>,
    OE: ScaleFactor + DivScale<OE1>,
    LE1: ScaleFactor,
    ME1: ScaleFactor,
    TE1: ScaleFactor,
    IE1: ScaleFactor,
    OE1: ScaleFactor,
{
    type Output = Qt<T, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;

    fn div(self, other: Qt<T, LE1, ME1, TE1, IE1, OE1>) -> Self::Output {
        Self::Output::new(self.value / other.value)
    }
}

//------------------------- Neg -------------------------

impl<T, LE, ME, TE, IE, OE> std::ops::Neg for Qt<T, LE, ME, TE, IE, OE>
where
    T: Number + std::ops::Neg<Output = T>,
    LE: ScaleFactor,
    ME: ScaleFactor,
    TE: ScaleFactor,
    IE: ScaleFactor,
    OE: ScaleFactor,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.value)
    }
}

//------------------------- Pow2 -------------------------

impl<T, LE, ME, TE, IE, OE> Pow2 for Qt<T, LE, ME, TE, IE, OE>
where
    T: Number + Pow2<Output = T>,
    LE: ScaleFactor + PowScale<2>,
    ME: ScaleFactor + PowScale<2>,
    TE: ScaleFactor + PowScale<2>,
    IE: ScaleFactor + PowScale<2>,
    OE: ScaleFactor + PowScale<2>,
{
    type Output = Qt<T, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;
    fn pow2(&self) -> <Self as Pow2>::Output {
        <Self as Pow2>::Output::new(self.value().pow2())
    }
}

//------------------------- Pow3 -------------------------

impl<T, LE, ME, TE, IE, OE> Pow3 for Qt<T, LE, ME, TE, IE, OE>
where
    T: Number + Pow3<Output = T>,
    LE: ScaleFactor + PowScale<3>,
    ME: ScaleFactor + PowScale<3>,
    TE: ScaleFactor + PowScale<3>,
    IE: ScaleFactor + PowScale<3>,
    OE: ScaleFactor + PowScale<3>,
{
    type Output = Qt<T, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;
    fn pow3(&self) -> <Self as Pow3>::Output {
        <Self as Pow3>::Output::new(self.value().pow3())
    }
}

//------------------------- Root2 -------------------------

impl<T, LE, ME, TE, IE, OE> Root2 for Qt<T, LE, ME, TE, IE, OE>
where
    T: Number + Root2<Output = T>,
    LE: ScaleFactor + RootScale<2>,
    ME: ScaleFactor + RootScale<2>,
    TE: ScaleFactor + RootScale<2>,
    IE: ScaleFactor + RootScale<2>,
    OE: ScaleFactor + RootScale<2>,
{
    type Output = Qt<T, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;
    fn root2(&self) -> <Self as Root2>::Output {
        <Self as Root2>::Output::new(self.value().root2())
    }
}

//------------------------- Root3 -------------------------

impl<T, LE, ME, TE, IE, OE> Root3 for Qt<T, LE, ME, TE, IE, OE>
where
    T: Number + Root3<Output = T>,
    LE: ScaleFactor + RootScale<3>,
    ME: ScaleFactor + RootScale<3>,
    TE: ScaleFactor + RootScale<3>,
    IE: ScaleFactor + RootScale<3>,
    OE: ScaleFactor + RootScale<3>,
{
    type Output = Qt<T, LE::Output, ME::Output, TE::Output, IE::Output, OE::Output>;
    fn root3(&self) -> <Self as Root3>::Output {
        <Self as Root3>::Output::new(self.value().root3())
    }
}
