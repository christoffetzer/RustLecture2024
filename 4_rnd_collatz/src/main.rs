use rand::Rng;

fn main() {
    // Random number generator
    let mut rng = rand::thread_rng();
    for i in 1..=2000 {
        // Generate random positive number
        let mut n = rng.gen::<u64>() + 1;
        print!("Iteration {i}: {n}");
        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = 3 * n + 1;
            }
            print!(" -> {n}");
        }
        println!(".");
    }
}
