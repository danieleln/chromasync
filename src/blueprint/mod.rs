mod parse_color;
mod parse_directive;

use crate::colortable::ColorTable;
use crate::config::blueprint::directive;
use crate::config::environ::{CACHE_BLUEPRINTS_DIR, CONFIG_BLUEPRINTS_DIR, POST_EXEC_SCRIPT};
use crate::logging::{
    log_as_error, Error, Error::BlueprintError, Error::ExecError, Error::SystemError,
};
use parse_color::parse_color;
use parse_directive::Directive;
use std::fs::{read_dir, DirEntry, File};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::Command;

pub fn build_blueprints(colors: &mut ColorTable) -> Result<(), Error> {
    // Looks for blueprints in both cache and config directories
    for dir in &[&*CONFIG_BLUEPRINTS_DIR, &*CACHE_BLUEPRINTS_DIR] {
        // Reads the content of the directory
        match read_dir(dir) {
            Err(e) => log_as_error(SystemError(e.to_string())),
            Ok(blueprints) => {
                // Iterates over the content of each directory
                for blueprint in blueprints {
                    match blueprint {
                        Err(e) => log_as_error(SystemError(e.to_string())),
                        Ok(blueprint) => {
                            // Checks if blueprint is a file and runs it
                            let result = build_blueprint_form_dir_entry(&blueprint, colors);
                            if let Err(BlueprintError(e)) = result {
                                log_as_error(BlueprintError(format!(
                                    "While parsing blueprint `{}`. {}",
                                    blueprint.path().display(),
                                    e
                                )));
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

fn build_blueprint_form_dir_entry(path: &DirEntry, colors: &mut ColorTable) -> Result<(), Error> {
    build_blueprint(&path.path(), colors)
}

pub fn build_blueprint(path: &PathBuf, colors: &mut ColorTable) -> Result<(), Error> {
    // Reads the content of the file,
    let file = File::open(path).map_err(|e| BlueprintError(e.to_string()))?;
    let reader = BufReader::new(file);

    // Default directive values
    let mut directives = Directive::new();

    let mut blueprint_instance = String::new();

    // Parses directives and colors
    let mut parsing_directive = true;
    for line in reader.lines() {
        let line = line.map_err(|e| BlueprintError(e.to_string()))?;

        // Parses directives only at the very beginning of the file
        if parsing_directive && !line.starts_with(directive::PREFIX) {
            parsing_directive = false;
        }

        if parsing_directive {
            directives.parse(&line).map_err(|e| BlueprintError(e))?;
        } else {
            blueprint_instance.push_str(&parse_color(&line, colors, &directives, &path));
            blueprint_instance.push_str("\n");
        }
    }

    // Writes the blueprint instance to a file
    let out_dir = directives.output_directory;
    let file_name = path.file_name().ok_or(SystemError(format!(
        "Missing file name `{}`",
        path.display()
    )))?;
    let out_file = out_dir.join(file_name);
    let mut file = File::create(out_file).map_err(|e| BlueprintError(e.to_string()))?;

    file.write_all(blueprint_instance.as_bytes())
        .map_err(|e| BlueprintError(e.to_string()))?;

    Ok(())
}
