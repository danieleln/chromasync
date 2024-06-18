mod cli;
mod config;
mod logging;

use logging::{log_as_error, Error};

fn main() {
    let matches = cli::build_parser().try_get_matches().map_err(|e| e.into());

    match matches {
        Err(e) => log_as_error(e),
        Ok(_) => {}
    }
}
