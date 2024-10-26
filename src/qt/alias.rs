use crate::*;

pub struct SI;

impl SI {
    //------------------------- Convert -------------------------

    // pub fn convert<T: Number + ToBase, P1: Prefix, P2: Prefix>(
    //     value: Metre_<T, P1>,
    // ) -> Metre_<T, P2> {
    //     Metre_::<T, P2>::new(value.value().to_base::<Exp1<P1>, P2>())
    // }

    //------------------------- Value -------------------------

    pub fn value<T: Number>(value: T) -> Value<T> {
        Value::new(value)
    }

    //------------------------- Meter -------------------------

    pub fn _m<T: Number, P: Prefix>(value: T) -> Metre_<T, P> {
        Metre_::new(value)
    }
    pub fn m<T: Number>(value: T) -> Metre<T> {
        Metre_::new(value)
    }
    pub fn km<T: Number>(value: T) -> KiloMetre<T> {
        Metre_::new(value)
    }
    pub fn dm<T: Number>(value: T) -> DeciMetre<T> {
        Metre_::new(value)
    }
    pub fn cm<T: Number>(value: T) -> CentiMetre<T> {
        Metre_::new(value)
    }
    pub fn mm<T: Number>(value: T) -> MilliMetre<T> {
        Metre_::new(value)
    }

    //------------------------- Gram -------------------------

    pub fn _g<T: Number, P: Prefix>(value: T) -> Gram_<T, P> {
        Gram_::new(value)
    }
    pub fn kg<T: Number>(value: T) -> KiloGram<T> {
        Gram_::new(value)
    }
    pub fn g<T: Number>(value: T) -> Gram<T> {
        Gram_::new(value)
    }

    //------------------------- Second -------------------------

    pub fn _s<T: Number, P: Prefix>(value: T) -> Second_<T, P> {
        Second_::new(value)
    }
    pub fn s<T: Number>(value: T) -> Second<T> {
        Second::new(value)
    }
    pub fn ms<T: Number>(value: T) -> MilliSecond<T> {
        MilliSecond::new(value)
    }
}

//------------------------- Value -------------------------

pub type Value<T> = Qt<
    T,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
>;

//------------------------- Meter -------------------------

pub type Metre__<T, E> =
    Qt<T, E, Scale<One, Exp<0>>, Scale<One, Exp<0>>, Scale<One, Exp<0>>, Scale<One, Exp<0>>>;
pub type Metre_<T, P> = Metre__<T, Scale<P, Exp<1>>>;
pub type Metre2_<T, P> = Metre__<T, Scale<P, Exp<2>>>;

pub type Metre<T> = Metre_<T, One>;

pub type KiloMetre<T> = Metre_<T, Kilo>;
pub type DeciMetre<T> = Metre_<T, Deci>;
pub type CentiMetre<T> = Metre_<T, Centi>;
pub type MilliMetre<T> = Metre_<T, Milli>;

//------------------------- Velocity -------------------------

// pub type Velocity_<T, PL, PT> =
//     Qt<T, Exp1<PL>, Scale<One, Exp<0>>, Exp_1<PT>, Scale<One, Exp<0>>, Scale<One, Exp<0>>>;

//------------------------- Gram -------------------------

pub type Gram_<T, P> = Qt<
    T,
    Scale<One, Exp<0>>,
    Scale<P, Exp<1>>,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
>;

pub type Gram<T> = Gram_<T, One>;
pub type KiloGram<T> = Gram_<T, Kilo>;

//------------------------- Second -------------------------

pub type Second_<T, P> = Qt<
    T,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
    Scale<P, Exp<1>>,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
>;

pub type Second<T> = Second_<T, One>;

pub type MilliSecond<T> = Second_<T, Milli>;
pub type MicroSecond<T> = Second_<T, Micro>;
pub type NanoSecond<T> = Second_<T, Nano>;

//------------------------- Ampere -------------------------

pub type Ampere<T> = Qt<
    T,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
    Scale<One, Exp<1>>,
    Scale<One, Exp<0>>,
>;

//------------------------- Kelvin -------------------------

pub type Kelvin<T> = Qt<
    T,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
    Scale<One, Exp<1>>,
>;

//------------------------- Newton = m⋅kg⋅s-2 -------------------------

// pub type Newton<T> =
//     Qt<T, Scale<One, Exp<1>>, Scale<Kilo, Exp<1>>, Exp_2<One>, Scale<One, Exp<0>>, Scale<One, Exp<0>>>;

//------------------------- Pascal = m-1⋅kg⋅s-2 -------------------------

// pub type Pascal<T> =
//     Qt<T, Exp_1<One>, Scale<Kilo, Exp<1>>, Exp_2<One>, Scale<One, Exp<0>>, Scale<One, Exp<0>>>;

//------------------------- Volt = m2⋅kg.s-3⋅A-1 -------------------------

// pub type Volt<T> = Qt<T, Exp2<One>, Scale<Kilo, Exp<1>>, Exp_3<One>, Exp_1<One>, Scale<One, Exp<0>>>;

//------------------------- Joule = m2⋅kg⋅s-2 -------------------------

// pub type Joule<T> =
//     Qt<T, Exp2<One>, Scale<Kilo, Exp<1>>, Exp_2<One>, Scale<One, Exp<0>>, Scale<One, Exp<0>>>;

//------------------------- Watt = m2⋅kg⋅s-3  -------------------------

// pub type Watt<T> =
//     Qt<T, Exp2<One>, Scale<Kilo, Exp<1>>, Exp_3<One>, Scale<One, Exp<0>>, Scale<One, Exp<0>>>;

//------------------------- Hertz = s-1  -------------------------

pub type Hertz<T> = Qt<
    T,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
    Scale<One, Exp<-1>>,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
>;

//------------------------- Coulomb = s.A  -------------------------

pub type Coulomb<T> = Qt<
    T,
    Scale<One, Exp<0>>,
    Scale<One, Exp<0>>,
    Scale<One, Exp<1>>,
    Scale<One, Exp<1>>,
    Scale<One, Exp<0>>,
>;

//------------------------- Ohm = m2⋅kg⋅s−3⋅A−2  -------------------------

// pub type Ohm<T> = Qt<T, Exp2<One>, Scale<Kilo, Exp<1>>, Exp_3<One>, Exp_2<One>, Scale<One, Exp<0>>>;

//------------------------- Velocity = m.s-1  -------------------------

// pub type Velocity<T> = Qt<
//     T,
//     Scale<One, Exp<1>>,
//     Scale<One, Exp<0>>,
//     Exp_1<One>,
//     Scale<One, Exp<0>>,
//     Scale<One, Exp<0>>,
// >;

//------------------------- Acceleration = m.s-2  -------------------------

// pub type Acceleration<T> = Qt<
//     T,
//     Scale<One, Exp<1>>,
//     Scale<One, Exp<0>>,
//     Exp_2<One>,
//     Scale<One, Exp<0>>,
//     Scale<One, Exp<0>>,
// >;
