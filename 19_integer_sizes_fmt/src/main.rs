use std::mem::size_of;

fn main() {
    println!("Integer sizes and ranges:");
    println!("  i8: {:3} bytes ; {:3} bits", size_of::<i8>(), i8::BITS);
    println!("  u8: {:3} bytes ; {:3} bits", size_of::<u8>(), u8::BITS);
    println!(" i16: {:3} bytes ; {:3} bits", size_of::<i16>(), i16::BITS);
    println!(" u16: {:3} bytes ; {:3} bits", size_of::<u16>(), u16::BITS);
    println!(" i32: {:3} bytes ; {:3} bits", size_of::<i32>(), i32::BITS);
    println!(" u32: {:3} bytes ; {:3} bits", size_of::<u32>(), u32::BITS);
    println!(" i64: {:3} bytes ; {:3} bits", size_of::<i64>(), i64::BITS);
    println!(" u64: {:3} bytes ; {:3} bits", size_of::<u64>(), u64::BITS);
    println!(
        "i128: {:3} bytes ; {:3} bits",
        size_of::<i128>(),
        i128::BITS
    );
    println!(
        "u128: {:3} bytes ; {:3} bits",
        size_of::<u128>(),
        u128::BITS
    );
}
