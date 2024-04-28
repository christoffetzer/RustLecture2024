use log::{error};
use clap_verbosity_flag::{ErrorLevel, Verbosity};


pub fn init_logger(verbose: &Verbosity<ErrorLevel>) {
    env_logger::Builder::new()
        .filter_level(verbose.log_level_filter())
        .init();
}

pub trait LogExt {
    fn log_msg(self, msg: &str) -> Self;
}

impl<T, E> LogExt for Result<T, E>
where
    E: std::fmt::Display,
{
    fn log_msg(self, msg: &str) -> Self {
        if let Err(e) = &self {
            error!("Error: {msg}: {e}");
        }
        self
    }
}

#[allow(dead_code)]
pub trait LogIgn {
    fn log_and_ignore(self, msg: &str);
}

impl<T, E> LogIgn for Result<T, E>
where
    E: std::fmt::Display,
{
    fn log_and_ignore(self, msg: &str) {
        if let Err(e) = &self {
            eprintln!("Error: {msg}: {e}");
        }
    }
}

