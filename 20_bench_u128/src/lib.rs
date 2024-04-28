pub fn collatz_u128(mut n: u128) -> u64 {
    let mut ret = 0;
    while n != 1 {
        ret += 1;
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1u128;
        }
    }
    ret
}
