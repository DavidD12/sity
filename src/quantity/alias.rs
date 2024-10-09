use super::*;

pub struct SI;

impl SI {
    //------------------------- Value -------------------------

    pub fn value<T: Copy>(value: T) -> Value<T> {
        Value::new(value)
    }

    //------------------------- Meter -------------------------

    pub fn _m<T: Copy, P: Prefix>(value: T) -> Metre_<T, P> {
        Metre_::new(value)
    }
    pub fn m<T: Copy>(value: T) -> Metre<T> {
        Metre_::new(value)
    }
    pub fn km<T: Copy>(value: T) -> KiloMetre<T> {
        Metre_::new(value)
    }
    pub fn dm<T: Copy>(value: T) -> DeciMetre<T> {
        Metre_::new(value)
    }
    pub fn cm<T: Copy>(value: T) -> CentiMetre<T> {
        Metre_::new(value)
    }
    pub fn mm<T: Copy>(value: T) -> MilliMetre<T> {
        Metre_::new(value)
    }

    //------------------------- Gram -------------------------

    pub fn _g<T: Copy, P: Prefix>(value: T) -> Gram_<T, P> {
        Gram_::new(value)
    }
    pub fn kg<T: Copy>(value: T) -> KiloGram<T> {
        Gram_::new(value)
    }
    pub fn g<T: Copy>(value: T) -> Gram<T> {
        Gram_::new(value)
    }

    //------------------------- Second -------------------------

    pub fn _s<T: Copy, P: Prefix>(value: T) -> Second_<T, P> {
        Second_::new(value)
    }
    pub fn s<T: Copy>(value: T) -> Second<T> {
        Second::new(value)
    }
    pub fn ms<T: Copy>(value: T) -> MilliSecond<T> {
        MilliSecond::new(value)
    }
}

//------------------------- Value -------------------------

pub type Value<T> = Quantity<T, Exp0, Exp0, Exp0, Exp0, Exp0>;

//------------------------- Meter -------------------------

pub type Metre_<T, P> = Quantity<T, Exp1<P>, Exp0, Exp0, Exp0, Exp0>;

pub type Metre<T> = Metre_<T, One>;

pub type KiloMetre<T> = Metre_<T, Kilo>;
pub type DeciMetre<T> = Metre_<T, Deci>;
pub type CentiMetre<T> = Metre_<T, Centi>;
pub type MilliMetre<T> = Metre_<T, Milli>;

//------------------------- Gram -------------------------

pub type Gram_<T, P> = Quantity<T, Exp0, Exp1<P>, Exp0, Exp0, Exp0>;

pub type Gram<T> = Gram_<T, One>;
pub type KiloGram<T> = Gram_<T, Kilo>;

//------------------------- Second -------------------------

pub type Second_<T, P> = Quantity<T, Exp0, Exp0, Exp1<P>, Exp0, Exp0>;

pub type Second<T> = Second_<T, One>;

pub type MilliSecond<T> = Second_<T, Milli>;
pub type MicroSecond<T> = Second_<T, Micro>;
pub type NanoSecond<T> = Second_<T, Nano>;

//------------------------- Ampere -------------------------

pub type Ampere<T> = Quantity<T, Exp0, Exp0, Exp0, Exp1<One>, Exp0>;

//------------------------- Kelvin -------------------------

pub type Kelvin<T> = Quantity<T, Exp0, Exp0, Exp0, Exp0, Exp1<One>>;

//------------------------- Newton = m⋅kg⋅s-2 -------------------------

pub type Newton<T> = Quantity<T, Exp1<One>, Exp1<Kilo>, Exp_2<One>, Exp0, Exp0>;

//------------------------- Pascal = m-1⋅kg⋅s-2 -------------------------

pub type Pascal<T> = Quantity<T, Exp_1<One>, Exp1<Kilo>, Exp_2<One>, Exp0, Exp0>;

//------------------------- Volt = m2⋅kg.s-3⋅A-1 -------------------------

pub type Volt<T> = Quantity<T, Exp2<One>, Exp1<Kilo>, Exp_3<One>, Exp_1<One>, Exp0>;

//------------------------- Joule = m2⋅kg⋅s-2 -------------------------

pub type Joule<T> = Quantity<T, Exp2<One>, Exp1<Kilo>, Exp_2<One>, Exp0, Exp0>;

//------------------------- Watt = m2⋅kg⋅s-3  -------------------------

pub type Watt<T> = Quantity<T, Exp2<One>, Exp1<Kilo>, Exp_3<One>, Exp0, Exp0>;

//------------------------- Hertz = s-1  -------------------------

pub type Hertz<T> = Quantity<T, Exp0, Exp0, Exp_1<One>, Exp0, Exp0>;

//------------------------- Coulomb = s.A  -------------------------

pub type Coulomb<T> = Quantity<T, Exp0, Exp0, Exp1<One>, Exp1<One>, Exp0>;

//------------------------- Ohm = m2⋅kg⋅s−3⋅A−2  -------------------------

pub type Ohm<T> = Quantity<T, Exp2<One>, Exp1<Kilo>, Exp_3<One>, Exp_2<One>, Exp0>;

//------------------------- Velocity = m.s-1  -------------------------

pub type Velocity<T> = Quantity<T, Exp1<One>, Exp0, Exp_1<One>, Exp0, Exp0>;

//------------------------- Acceleration = m.s-2  -------------------------

pub type Acceleration<T> = Quantity<T, Exp1<One>, Exp0, Exp_2<One>, Exp0, Exp0>;
