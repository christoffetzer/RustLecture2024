use termination_check::collatz_tc;

fn main() {
    let n = 29303209u128;
    println!("collatz_tc({n}) reaches 1 in {} steps", collatz_tc(n));
    let n = 0;
    println!("collatz_tc({n}) reaches 1 in {} steps", collatz_tc(n));
}
