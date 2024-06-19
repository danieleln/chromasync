use crate::colortable::ColorTable;
use crate::config::environ::{CACHE_BLUEPRINTS_DIR, CONFIG_BLUEPRINTS_DIR, POST_EXEC_SCRIPT};
use crate::logging::{log_as_error, Error, Error::BlueprintError, Error::ExecError};
use crate::util::is_file;
use clap::ArgMatches;
use std::fs::{read_dir, DirEntry, File};
use std::io::{Read, Write};
use std::process::Command;

pub fn build_blueprints(colors: &mut ColorTable, args: &ArgMatches) -> Result<(), Error> {
    // Looks for blueprints in both cache and config directories
    for dir in &[&*CONFIG_BLUEPRINTS_DIR, &*CACHE_BLUEPRINTS_DIR] {
        // Reads the content of the directory
        match read_dir(dir) {
            Err(e) => log_as_error(BlueprintError(e.to_string())),
            Ok(blueprints) => {
                // Iterates over the content of each directory
                for blueprint in blueprints {
                    match blueprint {
                        Err(e) => log_as_error(BlueprintError(e.to_string())),
                        Ok(blueprint) => {
                            // Checks if blueprint is a file and runs it
                            if is_file(&blueprint) {
                                build_blueprint(&blueprint, colors);
                            }
                        }
                    }
                }
            }
        }
    }

    // Runs chromasync-post.sh script
    Command::new(&*POST_EXEC_SCRIPT)
        .output()
        .map_err(|e| ExecError(e.to_string()))?;

    Ok(())
}

fn build_blueprint(blueprint: &DirEntry, colors: &mut ColorTable) -> Result<(), Error> {
    // Opens the file
    let mut blueprint = File::open(blueprint.path()).map_err(|e| BlueprintError(e.to_string()))?;

    // Reads its content
    let mut template = String::new();
    blueprint
        .read_to_string(&mut template)
        .map_err(|e| BlueprintError(e.to_string()))?;

    Ok(())
}
