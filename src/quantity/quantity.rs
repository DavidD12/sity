use super::*;
use crate::number::*;

pub trait Quantity: Number + HasValue {
    type Value: Number;
    type Length: ScaleFactor; // Length (m)
    type Mass: ScaleFactor; // Mass (kg)
    type Time: ScaleFactor; // Time (s)
    type Current: ScaleFactor; // Electric current (A)
    type Temperature: ScaleFactor; // Thermodynamic temperature (K)

    // Volt = m2⋅kg.s-3⋅A-1
    fn is_volt(&self) -> bool {
        Self::Length::PREFIX == 0
            && Self::Length::EXPONENT == 2
            && Self::Mass::PREFIX == 3
            && Self::Mass::EXPONENT == 1
            && Self::Time::PREFIX == 0
            && Self::Time::EXPONENT == -3
            && Self::Current::PREFIX == 0
            && Self::Current::EXPONENT == -1
            && Self::Temperature::EXPONENT == 0
    }

    // Hertz = s-1
    fn is_hertz(&self) -> bool {
        Self::Length::EXPONENT == 0
            && Self::Mass::EXPONENT == 0
            && Self::Time::PREFIX == 0
            && Self::Time::EXPONENT == -1
            && Self::Current::EXPONENT == 0
            && Self::Temperature::EXPONENT == 0
    }

    // Watt = m2⋅kg⋅s-3
    fn is_watt(&self) -> bool {
        Self::Length::PREFIX == 0
            && Self::Length::EXPONENT == 2
            && Self::Mass::PREFIX == 3
            && Self::Mass::EXPONENT == 1
            && Self::Time::PREFIX == 0
            && Self::Time::EXPONENT == -3
            && Self::Current::EXPONENT == 0
            && Self::Temperature::EXPONENT == 0
    }

    fn base_unit(&self) -> String {
        let mut sep = false;
        let mut s = String::new();
        // Length (m)
        if Self::Length::EXPONENT != 0 {
            s += &pretty::<Self::Length>("m");
            sep = true;
        }
        // Mass (Kg)
        if Self::Mass::EXPONENT != 0 {
            if sep {
                s += ".";
            }
            s += &pretty::<Self::Mass>("g");
            sep = true;
        }
        // Time (s)
        if Self::Time::EXPONENT != 0 {
            if sep {
                s += ".";
            }
            s += &pretty::<Self::Time>("s");
            sep = true;
        }
        // Electric (A)
        if Self::Current::EXPONENT != 0 {
            if sep {
                s += ".";
            }
            s += &pretty::<Self::Current>("A");
            sep = true;
        }
        // Temperature (K)
        if Self::Temperature::EXPONENT != 0 {
            if sep {
                s += ".";
            }
            s += &pretty::<Self::Temperature>("K");
            // sep = true;
        }
        s
    }

    fn unit(&self) -> String {
        if self.is_volt() {
            return "V".to_string();
        }
        if self.is_hertz() {
            return "Hz".to_string();
        }
        if self.is_watt() {
            return "W".to_string();
        }
        self.base_unit()
    }
}
