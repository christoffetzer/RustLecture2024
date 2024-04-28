fn main() {
    // Program entry point
    let mut n = 6; // Mutable variable binding
    print!("{n}"); // Macro for printing, like printf
    while n != 1 {
        // No parenthesis around expression
        if n % 2 == 0 {
            // Math like in other languages
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        print!(" -> {n}");
    }
    println!();
}
