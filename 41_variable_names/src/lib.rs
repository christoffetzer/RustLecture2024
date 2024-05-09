#![feature(let_chains)]

pub fn collatz(mut n: u64) -> u64 {
    let mut steps = 0;
    while n != 1 {
        steps += 1;
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1u64;
        }
    }
    steps
}

pub fn checked_collatz_0(mut n: u64) -> Option<u64> {
    let mut steps = 0u64;
    if n < 1 {
        // undefined if n == 0
        return None;
    }
    while n != 1 {
        steps += 1;
        if n % 2 == 0 {
            n /= 2;
        } else {
            let v = 3u64.checked_mul(n); // 3*n
            let v = if let Some(v) = v {
                // 3*n + 1
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

pub fn checked_collatz_1(mut n: u64) -> Option<u64> {
    let mut steps = 0u64;
    if n < 1 {
        // undefined if n == 0
        return None;
    }
    while n != 1 {
        steps += 1;
        if n % 2 == 0 {
            n /= 2;
        } else {
            let v1 = 3u64.checked_mul(n); // 3*n
            let v3 = if let Some(v2) = v1 {
                // 3*n + 1
                v2.checked_add(1u64)
            } else {
                None
            };
            n = match v3 {
                None => return None, // return None on overflow
                Some(v4) => v4,
            };
        }
    }
    Some(steps)
}

pub fn checked_collatz_2(mut n: u64) -> Option<u64> {
    let mut steps = 0u64;
    if n < 1 {
        // undefined if n == 0
        return None;
    }
    while n != 1 {
        steps += 1;
        if n % 2 == 0 {
            n /= 2;
        } else {
            // let chain ... not a stable feature
            if let Some(v) = n.checked_mul(3)
                && let Some(v) = v.checked_add(1)
            {
                n = v;
            } else {
                return None;
            };
        }
    }
    Some(steps)
}

pub fn checked_collatz_3(mut n: u64) -> Option<u64> {
    let mut steps = 0u64;
    if n < 1 {
        // undefined if n == 0
        return None;
    }
    while n != 1 {
        steps += 1;
        if n % 2 == 0 {
            n /= 2;
        } else {
            // let chain ... not a stable feature
            n = 3u64.checked_mul(n)?.checked_add(1)?
        }
    }
    Some(steps)
}
