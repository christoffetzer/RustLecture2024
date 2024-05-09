use rand::Rng;

// let us count how many steps we need to converge
// let us maintain a maximum number of steps

fn main() {
    let mut rng = rand::thread_rng();
    // let's maintain a maximum number
    let mut max_steps: u64 = 0;
    // let's try a lot random numbers
    for i in 1..=10_000_000 {
        // use u32 to represent u16 numbers to avoid overflow
        let mut x = 1u128 + rng.gen::<u64>() as u128;
        let mut steps: u64 = 0;
        print!("{i}: {x}");
        while x != 1 {
            steps += 1;
            if x % 2 == 0 {
                // we need one less bit after division
                x /= 2;
            } else {
                // we need about two extra bits for new x
                x = 3 * x + 1;
            }
        }
        println!(" -> {x} (steps: {steps})");
        if steps > max_steps {
            max_steps = steps;
        }
    }
    println!("maximum steps encountered: {max_steps}");
}
