use checked_collatz::checked_collatz;
use colored::Colorize;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut skipped = 0;
    let mut computed = 0;
    for i in 1..=2000 {
        let n = rng.gen::<u64>();
        if let Some(steps) = checked_collatz(n) {
            println!("Iteration {i}: Collatz({n}) reaches 1 in {steps} steps");
            computed += 1;
        } else {
            println!(
                "Iteration {i}: Collatz({n}) - {}",
                "skipping".to_string().red()
            );
            skipped += 1;
        }
    }
    println!(
        "Skipped {skipped} random numbers of a total of {}",
        skipped + computed
    );
}
