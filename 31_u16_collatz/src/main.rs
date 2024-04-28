
fn main() {
    // Random number generator
    for a in 1..=65535u16 {
        for b in 1..=65535u16 {
            // Generate random positive number 
            let mut n : u64 = a.into();
            let a_b_n = (b as u64) <<16 | a as u64;
            let mut a_b = a_b_n;
            // control flow is determined by the right hand side
            // this is complete determined by n
            // note there are special cases: 
            // n=0 (we skip this one -> we shift 16 to right)
            // n=1 (we skip this: 1 -> 4 -> shift 2 + shift right)
            // 00..0 => we shift by that number of bits
            while n != 1 {
                if n % 2 == 0 {
                    n /= 2;
                    a_b /= 2;
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
