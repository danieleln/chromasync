mod cli;
mod config;
mod logging;
mod subcommands;
mod util;

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
    set_verbosity(&args);

    // Runs the required subcommand
    let _ = match args.subcommand() {
        Some(("list", args)) => subcommands::list(args),
        Some(("load", args)) => subcommands::load(args),
        Some(("reload", args)) => subcommands::reload(args),
        _ => unreachable!(),
    }?;

    Ok(())
}
