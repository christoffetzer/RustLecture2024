fn main() {
    println!("Hello, world!");
    let zero = 0;
    let one = 1;
    let mut i = 1;
    println!("1234567890");
    while i > zero {
        i += &one; // to be symmetric with 34_*
                   // speed up output
        if i % 1024 == 0 {
            print!("{i}\r")
        }
    }
}
