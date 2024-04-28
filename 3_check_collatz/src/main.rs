fn main() {
    // check numbers n in {1, 2, … , 2000}
    for mut n in 1..=2000 {
        print!(": {n}"); 
        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = 3 * n + 1u32;
            }
            print!(" -> {n}");
        }
        println!();
    }
}
