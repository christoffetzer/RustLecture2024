fn main() {
    println!("Hello, world!");
    let mut i = 0;
    while i < 10 {
        i += 1;
        if i >= 10 {
            i = 1;
        }
        print!("{i}\r")
    }
}
