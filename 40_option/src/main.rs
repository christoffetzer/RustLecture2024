// we show some bad style .. hence allow needless_late_init
#[allow(clippy::needless_late_init)]
fn main() {
    let r1: Option<u64> = Some(1);
    let r2 = Some(2u64);
    let r3: Option<u64> = None;
    let r4: Option<u64>;

    r4 = Some(4);
    println!("Hello, option!\n - r1={r1:?}\n - r2={r2:?}\n - r3={r3:?}\n - r4={r4:?}");
}
