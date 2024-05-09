fn main() {
    // check numbers n in {1, 2, … , 2000}
    for mut n in 1..=65535u16 {
        println!(": {n}");
        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = 3 * n + 1;
            }
        }
    }
    println!("Done.")
}
