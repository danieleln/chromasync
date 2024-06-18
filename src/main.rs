mod cli;
mod config;
mod logging;

use logging::{log_as_error, set_verbosity, Error};

fn main() {
    let result = app();

    if let Err(e) = result {
        log_as_error(e);
    }
}

fn app() -> Result<(), Error> {
    // Parses input arguments
    let args = cli::build_parser()
        .try_get_matches()
        .map_err(|e| Error::from(e))?;

    // Builds project directories
    let _ = config::environ::build_dirs()?;

    // Sets logging verbosity
    set_verbosity(args);

    Ok(())
}
