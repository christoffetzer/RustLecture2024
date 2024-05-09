// by default, compiler tries to detect arithmetic overflows
// in program code. To be able to compile this code,
// we can permit arithmetic overflows as follows:

#[allow(arithmetic_overflow)]

fn main() {
    println!("Integer overflow:");
    // the following statement contains two arithmetic overflows:
    // - what is the result of the following output?
    println!("u8: [{},{}]", 0 - 1u8, 255u8 + 1);
}
