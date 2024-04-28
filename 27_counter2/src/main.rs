struct Counter {
    name: String,
    counter: u32,
}


fn as_string(cnt: &Counter) -> String {
    format!("Counter {}: value = {}", cnt.name, cnt.counter)
}


fn increment_(Counter) -> Counter {
    cnt.counter += 1;
    cnt
}

fn main() {
    let mut cnt = new_counter("my counter".to_string());
    cnt = increment(cnt);
    cnt = increment(cnt);
    println!("Counter = {}", as_string(&cnt));
}

fn f()
{
    let cnt_ref1 = &(*cnt); // immutable borrow
    let _cnt_ref3 = &mut (*cnt); // mutable borrow
      as_string(cnt_ref1);

}

struct Counter {
    name: String,
    counter: u32,
}

fn new_counter(name: String) -> Counter {
    Counter { name: name, counter: 0 }
}

fn increment(cnt: &mut Counter) {
// increment 
    (*cnt).counter += 1
}

fn as_string(cnt: &Counter) -> String {
// increment counter
    format!("Counter {}: value = {}", cnt.name, cnt.counter)
}
struct Counter {
    name: String,
    counter: u32,
}

fn new_counter(name: String) -> Counter {
    Counter { name: name, counter: 0 }
}

fn increment(cnt: &mut Counter) {
// increment 
    (*cnt).counter += 1
}

fn as_string(cnt: &Counter) -> String {
// increment counter
    format!("Counter {}: value = {}", cnt.name, cnt.counter)
}

struct Counter {
    name: String,
    counter: u32,
}

fn new_counter(name: String) -> Counter {
    Counter { name: name, counter: 0 }
}

fn increment(cnt: &mut Counter) {
// increment 
    (*cnt).counter += 1
}

fn as_string(cnt: &Counter) -> String {
// increment counter
    format!("Counter {}: value = {}", cnt.name, cnt.counter)
}

fn main() {
    let cnt = &mut new_counter("my counter".to_string());
    increment(cnt);
    increment(cnt);

    let cnt_ref1 = &(*cnt); 
    let cnt_ref2 = &(*cnt); 
    as_string(cnt_ref1);
    as_string(cnt_ref2);
    increment(cnt);
    let cnt_ref3 = &mut (*cnt);
    increment(cnt_ref3);

    increment(cnt);

    println!("Counter = {}", as_string(cnt));
}


