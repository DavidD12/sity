pub use sity::*;

fn main() {
    let m = SI::_m::<_, Centi>(1.0);
    let s = Second::new(2.0);
    let x = m * m / s;
    println!("x = {}", x);
}
