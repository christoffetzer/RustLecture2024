#![feature(exitcode_exit_method)]

use std::panic;
use std::thread;
use std::time::Duration;
use std::process::{ExitCode};


fn main() {
    panic::set_hook(Box::new(|x| {
        println!("Custom panic hook {x:?}");
        match x.payload().downcast_ref::<u8>() {
        Some(as_u8) => {
            println!("u8 ({as_u8})");
            ExitCode::from(*as_u8).exit_process();
        }
        None => {
            println!("Not a u8");
        }
    }
    ExitCode::from(42).exit_process();
    }));
    
    thread::spawn(|| {
            for i in 1..=3 {
                println!("Number {} - spawned thread", i);
                thread::sleep(Duration::from_millis(1));
            }
            std::panic::panic_any(4u8)
    });
    
    for i in 1..=10 {
            println!("Number {} from main thread!", i);
            thread::sleep(Duration::from_millis(1));
    }
}

