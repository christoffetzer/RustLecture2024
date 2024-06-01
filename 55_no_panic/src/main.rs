use no_panic::no_panic;

// the following code might panic!
// Hence, we cannot set #[no_panic]
#[allow(dead_code)]
fn div(n: i32, m: i32) -> i32 {
        m/n
}

// compiler proves that this cannot panic!
// note: need to compile with --release
#[no_panic]
fn div_p(n: i32, m: i32) -> Option<i32> {
    if n == 0 {
        None
    } else {
        Some(m/n)
    }
}

// compiler proves that this cannot panic!
// note: need to compile with --release
#[no_panic]
fn div_assert(n: i32, m: i32) -> Option<i32> {
    if n == 0 {
        None
    } else {
        assert!(n != 0);
        Some(m/n)
    }
}

// assert is not removed during release - to prove this, we check
// if n != 0 - this code panics iff the assert! is NOT removed
#[no_panic]
fn div_assert2(n: i32, m: i32) -> Option<i32> {
    assert!(n != 0);
    Some(m+n)
}


// println! cannot be proven to have no !panic
// #[no_panic]
fn print_it() {
    let vec = vec![1, 2, 3, 4, 5];
    for &item in &vec {
        println!("{}", item);
    }    
}

// compiler cannot prove the following
// which contains a range iterator
//#[no_panic]
fn sum_it_iter(vec: &Vec<i32>) -> i32 {
    let mut r = 0;
    for i in 0..vec.len() {
        r += vec[i];
    }    
    r
}


// running with the iterator of the vector,
// the compiler can prove that this will not
// panic!
#[no_panic]
fn sum_it(vec: &Vec<i32>) -> i32 {
    let mut r = 0;
    for &item in vec {
        r += item;
    }    
    r
}

// iterate over array
#[no_panic]
fn sum_array(array: &[i32]) -> i32 {
    let mut r = 0;
    for &item in array {
        r += item;
    }    
    r
}


// compiler cannot prove no_panic for the following code :-(
// #[no_panic]
fn f() {
    let n = 10;
    let m = 0;
    let r = div_p(n, m);
    println!("{n}/{m}={r:?}");
}

#[derive(Clone,Debug)]
#[allow(dead_code)]
enum HttpResultCode {
    Ok = 200,
    NotFound = 404,
    Teapot = 418,
}

// compiler can prove that assert_eq! are true
// in this case, that code is indeed 404
#[no_panic]
fn check_assertions() -> HttpResultCode {
    let code = HttpResultCode::NotFound;
    assert_eq!(code.clone() as i32, 404); // using other value like 200 would panic!
    code
}

// compiler cannot prove no_panic for the following code :-(
// #[no_panic]
// also, this program only compiles with --release - otherwise, the compiler 
// cannot prove that the code cannot panic!
fn main() {
    let n = 10;
    let m = 0;
    let r = div_p(n, m);
    println!("{n}/{m}={r:?}");
    f();
    let vec = vec![1, 2, 3, 4, 5];
    println!("sum={r}", r = sum_it(&vec));
    println!("sum={r}", r = sum_it_iter(&vec));
    print_it();
    println!("code={:?}", check_assertions());

    let a = [1,2,3,4,5];
    println!("sum({a:?}={}", sum_array(&a));
    div_assert(5,2);
    div_assert(0,0);
    // assert_eq is not removed during release - to prove this, uncomment the next statement
    // div_assert2(0,0);
}
