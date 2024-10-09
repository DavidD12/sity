pub use sity::*;
use std::mem;

fn main() {
    let a = SI::cm(3.0);
    let b = SI::cm(4.0);
    let c = SI::s(2.0);
    let si_value = a * b / c;
    let si_value_size = mem::size_of_val(&si_value);
    // let x = a + c; // Compile time Error

    println!("si_value = {} (size = {})", si_value, si_value_size);

    let pure_value = si_value.value();
    let pure_value_size = mem::size_of_val(&pure_value);
    println!("pure_value = {} (size = {})", pure_value, pure_value_size);
}
