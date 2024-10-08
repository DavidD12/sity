use super::*;
use std::fmt::Display;

//------------------------- Value = . -------------------------

pub type Value<T> = Quantity<T, Exp0, Exp0, Exp0, Exp0, Exp0>;
impl<T: Copy + Display> Symbol for Value<T> {
    fn symbol() -> &'static str {
        ""
    }
}

//------------------------- Meter -------------------------

pub type Metre<T> = Quantity<T, Exp1, Exp0, Exp0, Exp0, Exp0>;
impl<T: Copy + Display> Symbol for Metre<T> {
    fn symbol() -> &'static str {
        "m"
    }
}
pub type Metre2<T> = Quantity<T, Exp2, Exp0, Exp0, Exp0, Exp0>;
impl<T: Copy + Display> Symbol for Metre2<T> {
    fn symbol() -> &'static str {
        "m²"
    }
}
pub type Metre3<T> = Quantity<T, Exp3, Exp0, Exp0, Exp0, Exp0>;
impl<T: Copy + Display> Symbol for Metre3<T> {
    fn symbol() -> &'static str {
        "m³"
    }
}

//------------------------- KiloGram -------------------------

pub type KiloGram<T> = Quantity<T, Exp0, Exp1, Exp0, Exp0, Exp0>;
impl<T: Copy + Display> Symbol for KiloGram<T> {
    fn symbol() -> &'static str {
        "kg"
    }
}

//------------------------- Second -------------------------

pub type Second<T> = Quantity<T, Exp0, Exp0, Exp1, Exp0, Exp0>;
impl<T: Copy + Display> Symbol for Second<T> {
    fn symbol() -> &'static str {
        "s"
    }
}

//------------------------- Ampere -------------------------

pub type Ampere<T> = Quantity<T, Exp0, Exp0, Exp0, Exp1, Exp0>;
impl<T: Copy + Display> Symbol for Ampere<T> {
    fn symbol() -> &'static str {
        "A"
    }
}

//------------------------- Kelvin -------------------------

pub type Kelvin<T> = Quantity<T, Exp0, Exp0, Exp0, Exp0, Exp1>;
impl<T: Copy + Display> Symbol for Kelvin<T> {
    fn symbol() -> &'static str {
        "K"
    }
}

//------------------------- Newton = m⋅kg⋅s-2 -------------------------

pub type Newton<T> = Quantity<T, Exp1, Exp1, Exp_2, Exp0, Exp0>;
impl<T: Copy + Display> Symbol for Newton<T> {
    fn symbol() -> &'static str {
        "N"
    }
}

//------------------------- Pascal = m-1⋅kg⋅s-2 -------------------------

pub type Pascal<T> = Quantity<T, Exp_1, Exp1, Exp_2, Exp0, Exp0>;
impl<T: Copy + Display> Symbol for Pascal<T> {
    fn symbol() -> &'static str {
        "Pa"
    }
}

//------------------------- Volt = m2⋅kg.s-3⋅A-1 -------------------------

pub type Volt<T> = Quantity<T, Exp2, Exp1, Exp_3, Exp_1, Exp0>;
impl<T: Copy + Display> Symbol for Volt<T> {
    fn symbol() -> &'static str {
        "V"
    }
}

//------------------------- Joule = m2⋅kg⋅s-2 -------------------------

pub type Joule<T> = Quantity<T, Exp2, Exp1, Exp_2, Exp0, Exp0>;
impl<T: Copy + Display> Symbol for Joule<T> {
    fn symbol() -> &'static str {
        "J"
    }
}

//------------------------- Watt = m2⋅kg⋅s-3  -------------------------

pub type Watt<T> = Quantity<T, Exp2, Exp1, Exp_3, Exp0, Exp0>;
impl<T: Copy + Display> Symbol for Watt<T> {
    fn symbol() -> &'static str {
        "W"
    }
}

//------------------------- Hertz = s-1  -------------------------

pub type Hertz<T> = Quantity<T, Exp0, Exp0, Exp_1, Exp0, Exp0>;
impl<T: Copy + Display> Symbol for Hertz<T> {
    fn symbol() -> &'static str {
        "Hz"
    }
}

//------------------------- Coulomb = s.A  -------------------------

pub type Coulomb<T> = Quantity<T, Exp0, Exp0, Exp1, Exp1, Exp0>;
impl<T: Copy + Display> Symbol for Coulomb<T> {
    fn symbol() -> &'static str {
        "C"
    }
}

//------------------------- Ohm = m2⋅kg⋅s−3⋅A−2  -------------------------

pub type Ohm<T> = Quantity<T, Exp2, Exp1, Exp_3, Exp_2, Exp0>;
impl<T: Copy + Display> Symbol for Ohm<T> {
    fn symbol() -> &'static str {
        "Ω"
    }
}

//------------------------- Velocity = m.s-1  -------------------------

pub type Velocity<T> = Quantity<T, Exp1, Exp0, Exp_1, Exp0, Exp0>;
impl<T: Copy + Display> Symbol for Velocity<T> {
    fn symbol() -> &'static str {
        "m.s-1"
    }
}

//------------------------- Acceleration = m.s-2  -------------------------

pub type Acceleration<T> = Quantity<T, Exp1, Exp0, Exp_2, Exp0, Exp0>;
impl<T: Copy + Display> Symbol for Acceleration<T> {
    fn symbol() -> &'static str {
        "m.s-2"
    }
}
