use big_uint::collatz_big_uint;

fn main() {
    let n = 29303209u128; 
    println!("collatz_big_uint({n}) reaches 1 in {} steps", collatz_big_uint(n));
}
