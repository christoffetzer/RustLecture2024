use checked_collatz_2::*;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    for i in 1..=2000 {
        let n = rng.gen::<u64>();
        println!("Iternation {i}: Collatz({n}) returns {result:?}", result=checked_collatz_3(n));
    }
}
