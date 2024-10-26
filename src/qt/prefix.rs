use sealed::sealed;

#[sealed]
pub trait Prefix: Copy {
    const SYMBOL: &'static str;
    const PREFIX: i32;
}

#[derive(Copy, Clone, Debug)]
pub struct Yocto;
#[sealed]
impl Prefix for Yocto {
    const SYMBOL: &'static str = "y";
    const PREFIX: i32 = -24;
}

#[derive(Copy, Clone, Debug)]
pub struct Zepto;
#[sealed]
impl Prefix for Zepto {
    const SYMBOL: &'static str = "z";
    const PREFIX: i32 = -21;
}

#[derive(Copy, Clone, Debug)]
pub struct Atto;
#[sealed]
impl Prefix for Atto {
    const SYMBOL: &'static str = "a";
    const PREFIX: i32 = -18;
}

#[derive(Copy, Clone, Debug)]
pub struct Femto;
#[sealed]
impl Prefix for Femto {
    const SYMBOL: &'static str = "f";
    const PREFIX: i32 = -15;
}

#[derive(Copy, Clone, Debug)]
pub struct Pico;
#[sealed]
impl Prefix for Pico {
    const SYMBOL: &'static str = "p";
    const PREFIX: i32 = -12;
}

#[derive(Copy, Clone, Debug)]
pub struct Nano;
#[sealed]
impl Prefix for Nano {
    const SYMBOL: &'static str = "n";
    const PREFIX: i32 = -9;
}

#[derive(Copy, Clone, Debug)]
pub struct Micro;
#[sealed]
impl Prefix for Micro {
    const SYMBOL: &'static str = "μ";
    const PREFIX: i32 = -6;
}

#[derive(Copy, Clone, Debug)]
pub struct Milli;
#[sealed]
impl Prefix for Milli {
    const SYMBOL: &'static str = "m";
    const PREFIX: i32 = -3;
}

#[derive(Copy, Clone, Debug)]
pub struct Centi;
#[sealed]
impl Prefix for Centi {
    const SYMBOL: &'static str = "c";
    const PREFIX: i32 = -2;
}

#[derive(Copy, Clone, Debug)]
pub struct Deci;
#[sealed]
impl Prefix for Deci {
    const SYMBOL: &'static str = "d";
    const PREFIX: i32 = -1;
}

#[derive(Copy, Clone, Debug)]
pub struct One;
#[sealed]
impl Prefix for One {
    const SYMBOL: &'static str = "";
    const PREFIX: i32 = 0;
}

#[derive(Copy, Clone, Debug)]
pub struct Deca;
#[sealed]
impl Prefix for Deca {
    const SYMBOL: &'static str = "da";
    const PREFIX: i32 = 1;
}

#[derive(Copy, Clone, Debug)]
pub struct Hecto;
#[sealed]
impl Prefix for Hecto {
    const SYMBOL: &'static str = "h";
    const PREFIX: i32 = 2;
}

#[derive(Copy, Clone, Debug)]
pub struct Kilo;
#[sealed]
impl Prefix for Kilo {
    const SYMBOL: &'static str = "k";
    const PREFIX: i32 = 3;
}

#[derive(Copy, Clone, Debug)]
pub struct Mega;
#[sealed]
impl Prefix for Mega {
    const SYMBOL: &'static str = "M";
    const PREFIX: i32 = 6;
}

#[derive(Copy, Clone, Debug)]
pub struct Giga;
#[sealed]
impl Prefix for Giga {
    const SYMBOL: &'static str = "G";
    const PREFIX: i32 = 9;
}

#[derive(Copy, Clone, Debug)]
pub struct Tera;
#[sealed]
impl Prefix for Tera {
    const SYMBOL: &'static str = "T";
    const PREFIX: i32 = 12;
}

#[derive(Copy, Clone, Debug)]
pub struct Peta;
#[sealed]
impl Prefix for Peta {
    const SYMBOL: &'static str = "P";
    const PREFIX: i32 = 15;
}

#[derive(Copy, Clone, Debug)]
pub struct Exa;
#[sealed]
impl Prefix for Exa {
    const SYMBOL: &'static str = "E";
    const PREFIX: i32 = 18;
}

#[derive(Copy, Clone, Debug)]
pub struct Zetta;
#[sealed]
impl Prefix for Zetta {
    const SYMBOL: &'static str = "Z";
    const PREFIX: i32 = 21;
}

#[derive(Copy, Clone, Debug)]
pub struct Yotta;
#[sealed]
impl Prefix for Yotta {
    const SYMBOL: &'static str = "Y";
    const PREFIX: i32 = 24;
}
