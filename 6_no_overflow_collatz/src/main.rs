use num_bigint::BigUint;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    // Program entry point
    let zero = BigUint::from(0u32);
    let one = BigUint::from(1u32);
    let two = BigUint::from(2u32);
    let three = BigUint::from(3u32);
    let mut max_steps = 0;
    for i in 1..=20_000 {
        let mut n: BigUint = BigUint::from(rng.gen::<u128>() + 1);
        let mut steps: u64 = 0;
        print!("{i}: {n}");
        while n != one {
            steps += 1;
            if &n % &two == zero {
                // Math like in u128
                n /= &two;
            } else {
                n = (&three * n) + &one;
            }
        }
        println!(" -> {n} (steps: {steps})");
        if steps > max_steps {
            max_steps = steps;
        }
    }
    println!("maximum number of steps: {max_steps}")
}
