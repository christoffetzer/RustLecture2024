struct Counter {
    name: String,
    counter: u32,
}

fn new_counter(name: String) -> Counter {
    Counter {
        name,
        counter: 0,
    }
}

fn increment(cnt: &mut Counter) {
    // cnt not mutable
    cnt.counter += 1
}

fn increment2(cnt1: &mut Counter, cnt2: &mut Counter) {
    // example for mutable variable containing a reference to a mutable value 
    let mut cnt : &mut Counter = cnt1;
    increment(cnt);
    cnt = cnt2;
    increment(cnt);
}

fn as_string(cnt: &Counter) -> String {
    // increment counter
    format!("Counter {}: value = {}", cnt.name, cnt.counter)
}

fn main() {
    let cnt = &mut new_counter("my counter".to_string());
    let cnt2 = &mut new_counter("my counter 2".to_string());
    increment(cnt);
    increment(cnt);
    increment2(cnt, cnt2);

    let cnt_ref1 = &(*cnt);
    let cnt_ref2 = &(*cnt);
    as_string(cnt_ref1);
    as_string(cnt_ref2);
    increment(cnt);
    let cnt_ref3 = &mut (*cnt);
    increment(cnt_ref3);

    increment(cnt);

    println!("Counter = {}", as_string(cnt));
    println!("Counter2 = {}", as_string(cnt2));
}
