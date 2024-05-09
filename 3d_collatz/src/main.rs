use cbitmap::bitmap::*;

fn main() {
    // we seen that program does not terminate for 14563
    let mut n = 14563u16;
    print!("{n}");
    // we want to detect if we are in an infinite loop
    // this happens if we assign to n a value that
    // we have assigned previously
    let mut bitmap = newmap!(0b0; 65536);
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        print!(" -> {n}");
        // check if we have visited state n already
        if bitmap.test(n.into()) {
            println!("\nWe detected a cycle!");
            break;
        }
        // mark n as visited
        bitmap.set(n.into());
    }
    println!();
}
