mod blueprint;
mod cli;
mod color_test_table;
mod colortable;
mod config;
mod logging;
mod subcommands;
mod util;

use logging::{log_as_error, set_verbosity, Error};
use std::process;

fn main() {
    let result = app();

    if let Err(e) = result {
        match e {
            Error::HelpMessage(msg) => {
                println!("{}", msg);
            }
            msg => {
                log_as_error(msg);

                // FIX: add different error codes!
                process::exit(1);
            }
        }
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
        Some(("preview", args)) => subcommands::preview(args),
        _ => unreachable!(),
    }?;

    Ok(())
}
