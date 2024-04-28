use collatz::collatz;

fn main() {
    let n = 29303209; 
    println!("Collatz({n}) reaches 1 in {} steps", collatz(n));
}
