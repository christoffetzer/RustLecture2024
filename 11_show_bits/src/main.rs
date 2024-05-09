fn main() {
    let nums = vec![5, 7, 8, 12, 15, 127, -1, -128];
    for n in nums {
        let d = n / 2;
        let m = 3 * n + 1;
        println!("num:   {n:03}={n:#010b}");
        println!("n/2:   {d:03}={d:#010b}");
        println!("3*n+1: {m:03}={m:#010b}\n");
    }
}
