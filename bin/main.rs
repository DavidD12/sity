pub use sity::*;

fn main() {
    let x = Metre::new(1.0);
    let y = x * 3.0;
    let z: Metre2<_> = x.pow2() + 3.0 * x.pow2();
    println!("x = {}", z.sqrt());
}
