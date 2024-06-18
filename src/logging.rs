#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{}", .0)]
    InvalidCommandLineArgument(String),
}

impl From<clap::error::Error> for Error {
    fn from(e: clap::error::Error) -> Self {
        Error::InvalidCommandLineArgument(e.to_string().split_off(7))
    }
}

pub enum Level {
    Error,
    Warning,
    Info,
}

pub fn log_as(level: Level, error: Error) {
    let label = match level {
        Level::Error => "\x1b[31mERR",
        Level::Warning => "\x1b[33mWRN",
        Level::Info => "\x1b[32mNFO",
    };

    println!("[{}\x1b[0m] {}", label, error);
}

pub fn log_as_info(e: Error) {
    log_as(Level::Info, e);
}

pub fn log_as_warning(e: Error) {
    log_as(Level::Warning, e);
}

pub fn log_as_error(e: Error) {
    log_as(Level::Error, e);
}
