fn main() {
    // Random number generator
    for i in 1..=3 {
        for j in 1..=3 {
            // Generate random positive number
            let mut n = (i as u64) << 16 | j as u64;
            println!(">- {n:#064b}: {n}");
            while n != 1 {
                if n % 2 == 0 {
                    n /= 2;
                    println!("/2 {n:#064b}: {n}");
                } else {
                    println!("*3 {:#064b}: {n}", n * 2);
                    println!("   {n:#064b}");
                    n = 3 * n + 1;
                    println!("   {n:#064b}");
                }
            }
            println!(".");
        }
    }
}
