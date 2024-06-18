use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref VERBOSE: Mutex<bool> = Mutex::new(false);
    static ref QUIET: Mutex<bool> = Mutex::new(false);
}

pub fn set_verbosity(args: &clap::ArgMatches) {
    let mut verbose = VERBOSE.lock().unwrap();
    let mut quiet = QUIET.lock().unwrap();

    if args.get_flag("verbose") {
        *verbose = true;
        *quiet = false;
    } else if args.get_flag("quiet") {
        *quiet = true;
        *verbose = false;
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{}", .0)]
    InvalidCommandLineArgument(String),

    #[error("{}", .0)]
    InvalidEnvironment(String),
}

impl From<clap::error::Error> for Error {
    fn from(e: clap::error::Error) -> Self {
        Error::InvalidCommandLineArgument(e.to_string().split_off(7))
    }
}

#[derive(PartialEq)]
pub enum Level {
    Error,
    Warning,
    Info,
}

pub fn log_as(level: Level, error: Error) {
    let verbose = VERBOSE.lock().unwrap();
    let quiet = QUIET.lock().unwrap();

    if *quiet == true {
        return;
    }

    if level == Level::Info && *verbose == false {
        return;
    }

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
