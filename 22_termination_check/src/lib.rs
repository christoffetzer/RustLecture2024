use num_bigint::BigUint;
use std::collections::HashSet;
use std::process;


pub fn collatz_u128(mut n: u128) -> u64 {
    let mut ret = 0;
    while n != 1 {
        ret += 1;
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1u128;
        }
    }
    ret
}

pub fn collatz_tc(n: u128) -> u64 {
    let mut set: HashSet<BigUint> = HashSet::new();
    let mut n = BigUint::from(n);
    let zero: BigUint =  BigUint::from(0u32);
    let one =  BigUint::from(1u32);
    let two =  BigUint::from(2u32);
    let three =  BigUint::from(3u32);
    let mut steps: u64 = 0;
    while n != one {
        if set.contains(&n) {
            println!("Detected cycle {n} -> {n}");
            process::exit(0x0100);
        }
        set.insert(n.clone());
        steps += 1;
        if &n % &two == zero {
            n /= &two;
        } else {
            n = (&three * n) + &one;
        }
    }
    steps
}
