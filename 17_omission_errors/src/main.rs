use colored::*;
use rand::Rng;
use std::panic;
use std::process;

fn set_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            let err_msg = format!("{s:?}").red();
            eprintln!("A fatal error occurred: {err_msg} (Error 15643-6574-32326)");
        } else {
            let err_msg = format!("{panic_info}");
            eprintln!(
                "A fatal error occurred: {}. (Error 7408-12196-20880)",
                err_msg.trim_start_matches("panicked at ").red()
            );
        }
        process::exit(1);
    }));
}

// check for error .. using panic_hook ... difficult to recover

fn main() {
    set_panic_hook(); // issue a nicer error message
    let mut rng = rand::thread_rng();
    for i in 1..=2000 {
        let mut n = rng.gen::<u64>() + 1;
        print!("Iteration {i}: {n}");
        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = 3 * n + 1;
            }
            print!(" -> {n}");
        }
        println!(".");
    }
}
