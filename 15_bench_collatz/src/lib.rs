pub fn collatz(mut n: u64) -> u64 {
    let mut ret = 0;
    while n != 1 {
        ret += 1;
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1u64;
        }
    }
    ret
}
