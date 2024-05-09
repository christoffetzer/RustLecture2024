use collatz_u128::collatz_u128;

fn main() {
    let n = 29303209u128;
    println!("Collatz({n}) reaches 1 in {} steps", collatz_u128(n));
}
