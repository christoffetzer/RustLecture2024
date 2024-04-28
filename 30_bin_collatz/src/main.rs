use rand::Rng;

fn main() {
    // Random number generator
    let mut rng = rand::thread_rng();
    for _i in 1..=2000 {
        // Generate random positive number 
        let mut n = rng.gen::<u32>() as u64 + 1;
        println!(">- {n:#064b}: {n}");
        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
                println!("/2 {n:#064b}: {n}");
            } else {
                println!("*3 {:#064b}: {n}", n*2);
                println!("   {n:#064b}");
                n = 3 * n + 1;
                println!("   {n:#064b}");
            }
        }
        println!(".");
    }
}
