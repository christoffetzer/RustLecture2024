use num_bigint::BigUint;

fn main() {
    println!("Hello, world!");
    let zero = BigUint::from(0u32);
    let one = BigUint::from(1u32);
    let mut i = BigUint::from(1u32);
    println!("1234567890");
    while i > zero {
        i += &one;
        print!("{i}\r")
    }
}
