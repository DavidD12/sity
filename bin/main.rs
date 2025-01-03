pub use sity::*;

fn main() {
    // generate_scale_mul().unwrap();
    // generate_scale_div().unwrap();
    // generate_scale_pow().unwrap();
    // generate_scale_root().unwrap();

    // let x = metre(123.0);
    // let y: KiloMetre<_> = x.convert();

    // println!("{}", x);
    // println!("{}", y);

    // let x = metre(3.0);
    // let y: CentiMetre<_> = x.convert();
    // println!("{} = {}", x, y);

    let m = metre(1.0);
    let s = second(3.0);
    let x = m * m / s;
    println!("m = {}", m);
    println!("s = {}", s);
    println!("x = {}", x);

    // let g = kilo_gram(2.0);
    // let a = ampere(4.0);

    // let x = m.pow2() * g / s.pow3() / a;
    // println!("{}", x);
    // let y = volt(1.2);
    // println!("{}", y);

    let x = hertz(2.0);
    println!("{}", x);
    let x = x * x;
    println!("{}", x);
    // println!("{}", watt(3.0));

    // let b = centi_metre(4.0);
    // let c = second(2.0);
    // println!("a = {}", a);
    // println!("b = {}", b);
    // println!("c = {}", c.pow2().root2());
    // let si_value = a / c;
    // use std::mem;
    // let si_value_size = mem::size_of_val(&si_value);
    // // let x = a + c; // Compile time Error

    // println!("si_value = {} (size = {})", si_value, si_value_size);
    // let pure_value = si_value.value();
    // let pure_value_size = mem::size_of_val(&pure_value);
    // println!("pure_value = {} (size = {})", pure_value, pure_value_size);
}
