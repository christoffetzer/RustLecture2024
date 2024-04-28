use rand::Rng;
use num_bigint::BigUint;
use std::collections::HashSet;
use std::process;

fn main() {
    let mut rng = rand::thread_rng();

    // Program entry point
    let zero =  BigUint::from(0u32);
    let one =  BigUint::from(1u32);
    let two =  BigUint::from(2u32);
    let three =  BigUint::from(3u32);
    for i in 1..=20_000 {
        let mut sequence: HashSet<BigUint> = HashSet::new();
        let mut n: BigUint = BigUint::from(rng.gen::<u8>() + 1);
        print!("{i}: {n}");
        sequence.insert(three.clone());

        while n != one {
            if sequence.contains(&n) {
                println!("Detected cycle {n} -> {n}");
                process::exit(0x0100);
            }
            sequence.insert(n.clone());
            if &n % &two == zero {
                // Math like in u128
                n /= &two;
            } else {
                n = (&three * n) + &one;
            }
        }
        println!(" -> {n}");
    }
}
