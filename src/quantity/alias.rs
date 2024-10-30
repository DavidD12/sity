use crate::*;

//------------------------- Value -------------------------

pub type Value<T> =
    Qt<T, Scale<One, 0>, Scale<One, 0>, Scale<One, 0>, Scale<One, 0>, Scale<One, 0>>;

pub fn value<T: Scalar>(value: T) -> Value<T> {
    Value::new(value)
}

//------------------------- Meter -------------------------

pub type Metre__<T, E> = Qt<T, E, Scale<One, 0>, Scale<One, 0>, Scale<One, 0>, Scale<One, 0>>;
pub type Metre_<T, P> = Metre__<T, Scale<P, 1>>;
pub type Metre2_<T, P> = Metre__<T, Scale<P, 2>>;

pub type Metre2<T> = Metre__<T, Scale<One, 2>>;

pub type KiloMetre<T> = Metre_<T, Kilo>;
pub type HectoMetre<T> = Metre_<T, Hecto>;
pub type DecaMetre<T> = Metre_<T, Deca>;
pub type Metre<T> = Metre_<T, One>;
pub type DeciMetre<T> = Metre_<T, Deci>;
pub type CentiMetre<T> = Metre_<T, Centi>;
pub type MilliMetre<T> = Metre_<T, Milli>;

pub fn _m<T: Scalar, P: Prefix>(value: T) -> Metre_<T, P> {
    Metre_::new(value)
}

pub fn kilo_metre<T: Scalar>(value: T) -> KiloMetre<T> {
    Metre_::new(value)
}
pub fn hecto_metre<T: Scalar>(value: T) -> HectoMetre<T> {
    Metre_::new(value)
}
pub fn deca_metre<T: Scalar>(value: T) -> DecaMetre<T> {
    Metre_::new(value)
}
pub fn metre<T: Scalar>(value: T) -> Metre<T> {
    Metre_::new(value)
}
pub fn deci_metre<T: Scalar>(value: T) -> DeciMetre<T> {
    Metre_::new(value)
}
pub fn centi_metre<T: Scalar>(value: T) -> CentiMetre<T> {
    Metre_::new(value)
}
pub fn milli_metre<T: Scalar>(value: T) -> MilliMetre<T> {
    Metre_::new(value)
}

//------------------------- Gram -------------------------

pub type Gram_<T, P> =
    Qt<T, Scale<One, 0>, Scale<P, 1>, Scale<One, 0>, Scale<One, 0>, Scale<One, 0>>;

pub type Gram<T> = Gram_<T, One>;
pub type KiloGram<T> = Gram_<T, Kilo>;

pub fn _g<T: Scalar, P: Prefix>(value: T) -> Gram_<T, P> {
    Gram_::new(value)
}
pub fn kilo_gram<T: Scalar>(value: T) -> KiloGram<T> {
    Gram_::new(value)
}
pub fn gram<T: Scalar>(value: T) -> Gram<T> {
    Gram_::new(value)
}

//------------------------- Second -------------------------

pub type Second_<T, P> =
    Qt<T, Scale<One, 0>, Scale<One, 0>, Scale<P, 1>, Scale<One, 0>, Scale<One, 0>>;

pub type Second<T> = Second_<T, One>;

pub type MilliSecond<T> = Second_<T, Milli>;
pub type MicroSecond<T> = Second_<T, Micro>;
pub type NanoSecond<T> = Second_<T, Nano>;

pub fn _s<T: Scalar, P: Prefix>(value: T) -> Second_<T, P> {
    Second_::new(value)
}
pub fn second<T: Scalar>(value: T) -> Second<T> {
    Second::new(value)
}
pub fn milli_second<T: Scalar>(value: T) -> MilliSecond<T> {
    MilliSecond::new(value)
}

pub fn micro_second<T: Scalar>(value: T) -> MicroSecond<T> {
    MicroSecond::new(value)
}

pub fn nano_second<T: Scalar>(value: T) -> NanoSecond<T> {
    NanoSecond::new(value)
}

//------------------------- Ampere -------------------------

pub type Ampere<T> =
    Qt<T, Scale<One, 0>, Scale<One, 0>, Scale<One, 0>, Scale<One, 1>, Scale<One, 0>>;

pub fn ampere<T: Scalar>(value: T) -> Ampere<T> {
    Ampere::new(value)
}

//------------------------- Kelvin -------------------------

pub type Kelvin<T> =
    Qt<T, Scale<One, 0>, Scale<One, 0>, Scale<One, 0>, Scale<One, 0>, Scale<One, 1>>;

pub fn kelvin<T: Scalar>(value: T) -> Kelvin<T> {
    Kelvin::new(value)
}

//-------------------------------------------------- Derived --------------------------------------------------

//------------------------- Newton = m⋅kg⋅s-2 -------------------------

pub type Newton<T> =
    Qt<T, Scale<One, 1>, Scale<Kilo, 1>, Scale<One, -2>, Scale<One, 0>, Scale<One, 0>>;

pub fn newton<T: Scalar>(value: T) -> Newton<T> {
    Newton::new(value)
}

//------------------------- Pascal = m-1⋅kg⋅s-2 -------------------------

pub type Pascal<T> =
    Qt<T, Scale<One, -1>, Scale<Kilo, 1>, Scale<One, -2>, Scale<One, 0>, Scale<One, 0>>;

pub fn pascal<T: Scalar>(value: T) -> Pascal<T> {
    Pascal::new(value)
}

//------------------------- Volt = m2⋅kg.s-3⋅A-1 -------------------------

pub type Volt<T> =
    Qt<T, Scale<One, 2>, Scale<Kilo, 1>, Scale<One, -3>, Scale<One, -1>, Scale<One, 0>>;

pub fn volt<T: Scalar>(value: T) -> Volt<T> {
    Volt::new(value)
}

//------------------------- Joule = m2⋅kg⋅s-2 -------------------------

pub type Joule<T> =
    Qt<T, Scale<One, 2>, Scale<Kilo, 1>, Scale<One, -2>, Scale<One, 0>, Scale<One, 0>>;

pub fn joule<T: Scalar>(value: T) -> Joule<T> {
    Joule::new(value)
}
//------------------------- Watt = m2⋅kg⋅s-3  -------------------------

pub type Watt<T> =
    Qt<T, Scale<One, 2>, Scale<Kilo, 1>, Scale<One, -3>, Scale<One, 0>, Scale<One, 0>>;

pub fn watt<T: Scalar>(value: T) -> Watt<T> {
    Watt::new(value)
}

//------------------------- Hertz = s-1  -------------------------

pub type Hertz<T> =
    Qt<T, Scale<One, 0>, Scale<One, 0>, Scale<One, -1>, Scale<One, 0>, Scale<One, 0>>;

pub fn hertz<T: Scalar>(value: T) -> Hertz<T> {
    Hertz::new(value)
}

//------------------------- Coulomb = s.A  -------------------------

pub type Coulomb<T> =
    Qt<T, Scale<One, 0>, Scale<One, 0>, Scale<One, 1>, Scale<One, 1>, Scale<One, 0>>;

pub fn coulomb<T: Scalar>(value: T) -> Coulomb<T> {
    Coulomb::new(value)
}

//------------------------- Ohm = m2⋅kg⋅s−3⋅A−2  -------------------------

pub type Ohm<T> =
    Qt<T, Scale<One, 2>, Scale<Kilo, 1>, Scale<One, -3>, Scale<One, -2>, Scale<One, 0>>;

pub fn ohm<T: Scalar>(value: T) -> Ohm<T> {
    Ohm::new(value)
}

//------------------------- Velocity = m.s-1  -------------------------

pub type Velocity_<T, PL, PT> =
    Qt<T, Scale<PL, 1>, Scale<One, 0>, Scale<PT, -1>, Scale<One, 0>, Scale<One, 0>>;
pub type Velocity<T> =
    Qt<T, Scale<One, 1>, Scale<One, 0>, Scale<One, -1>, Scale<One, 0>, Scale<One, 0>>;

//------------------------- Acceleration = m.s-2  -------------------------

pub type Acceleration<T> =
    Qt<T, Scale<One, 1>, Scale<One, 0>, Scale<One, -2>, Scale<One, 0>, Scale<One, 0>>;
