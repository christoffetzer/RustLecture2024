use std::process;
use inline_colorization::*;
use num_traits::Unsigned;

// n in 0..T::BITS

fn generate_integer_with_n_ones<T: Unsigned + std::convert::From<u64> + std::ops::Shl<usize, Output = T> + std::ops::BitOr<Output = T> + Copy>(n: usize) -> T {
    let mask : T = T::from(1u64) << n;   // Convert 1 to T and shift it left by n
    let one : T = 1u64.into();
    mask | (mask - one)    // Subtract 1 from the mask and perform a bitwise OR
}

fn main() {
    // Random number generator
    for a in 1..=65535u16 {
        for b in 1..=65535u16 {
            // Generate random positive number 
            let mut n : u64 = a.into();
            let a_b_n = (b as u64) <<16 | a as u64;
            let mut a_b = a_b_n;
            let mut bits = 16;
            // control flow is determined by the right hand side
            // this is complete determined by n
            // note there are special cases: 
            // n=0 (we skip this one -> we shift 16 to right)
            // n=1 (we skip this: 1 -> 4 -> shift 2 + shift right)
            // 00..0 => we shift by that number of bits
            while n != 1 {
                if (a_b & n) == n {
                    println!("{n:064b} (bits: {})\n{color_green}{a_b:064b} (bits: {}){color_reset} ", 64-n.leading_zeros(), 64-a_b.leading_zeros());

                } else {
                    println!("{n:064b} (bits: {})\n{color_red}{a_b:064b} (bits: {}){color_reset} ", 64-n.leading_zeros(), 64-a_b.leading_zeros());

                }

                if (a_b % 2) != (n % 2) {
                    println!("{a} vs {b} vs {}: {n} vs {a_b}", (b as u64) <<16 | a as u64);
                    process::exit(0x0100);
                }
                if n % 2 == 0 {
                    n /= 2;
                    a_b /= 2;
                    bits -= 1;
                } else {
                    n = 3 * n + 1;
                    a_b = 3 * a_b + 1;
                    // check if there is a carry
                    // if we add +1 or +0

                }
            }
            if a_b > a_b_n  {
                println!("a={a}, b={b}, a_b={a_b}");
            }
        }
    }
}
