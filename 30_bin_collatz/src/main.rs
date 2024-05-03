use rand::Rng;
use inline_colorization::*;

fn main() {
    // Random number generator
    let mut rng = rand::thread_rng();
    for _i in 1..=1 {
        // Generate random positive number 
        let mut n = rng.gen::<u32>() as u64 + 1;
        n = 27;
        println!(">- {n:#064b}: {n}");
        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
                println!("/2 {color_green}{n:064b}{color_reset}: {n:04} bits: {}", 64-n.leading_zeros());
            } else {
                println!("*3 {:064b}: ", n*2);
                println!("   {n:064b}");
                n = 3 * n + 1;
                println!("   {color_red}{n:064b}{color_reset}: {n:04} bits: {}", 64-n.leading_zeros());
            }
        }
        println!(".");
    }
}
