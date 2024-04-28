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

pub fn checked_collatz(mut n: u64) -> Option<u64> {
    let mut steps = 0u64;
    if n < 1 { // undefined if n == 0
        return None
    }
    while n != 1 {
        steps += 1;
        if n % 2 == 0 {
            n /= 2;
        } else {
            let v = 3u64.checked_mul(n); // 3*n
            let v = if let Some(v) = v { // 3*n + 1
                v.checked_add(1u64)
            } else {
                None
            };
            n = match v {
                None => return None, // return None on overflow
                Some(v) => v,
            };
        }
    }
    Some(steps)
}

