pub mod prefix;
pub use prefix::*;

pub mod exponent;
pub use exponent::*;

pub mod exponent_div;
pub mod exponent_mul;
pub mod exponent_pow;
pub mod exponent_root;

pub mod scale_factor;
pub use scale_factor::*;

pub mod quantity;
pub use quantity::*;

pub mod alias;
pub use alias::*;

//----------------------------------------------------------------------------------------------------

pub mod generator;
pub use generator::*;

//----------------------------------------------------------------------------------------------------

pub fn test() {
    // let x = Qt::<f64, Scale<One, Exp<1>>, Scale<Centi, Exp<1>>>::new(10.0);
    // let y = Qt::<f64, Scale<One, Exp<1>>, Scale<Centi, Exp<1>>>::new(20.0);
    // let z = x + y;
    // let z = x * y;
    // let z = x * 3.0 / y;
    // let z = x.pow2().root2();
}
