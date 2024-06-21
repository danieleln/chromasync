use crate::blueprint::{build_blueprint, build_blueprints};
use crate::colortable::ColorTable;
use crate::config::environ::{
    CACHE_BLUEPRINTS_DIR, CONFIG_BLUEPRINTS_DIR, CURRENT_COLORSCHEME_FILE, POST_EXEC_SCRIPT,
};
use crate::logging::log_as_error;
use crate::logging::Error::{self, BlueprintError, ExecError, SystemError};
use clap::ArgMatches;
use std::path::PathBuf;
use std::process::Command;

pub fn reload(args: &ArgMatches) -> Result<(), Error> {
    // Loads the colorscheme
    let mut colors = ColorTable::from_file_path(&*CURRENT_COLORSCHEME_FILE)
        .map_err(|e| Error::ColorschemeError(e))?;

    // Instantiates all the blueprints
    build_selected_blueprints(&args, &mut colors)?;

    // Runs chromasync-post.sh script only if flag --no-script is missing
    if args.get_one::<bool>("no-script").is_none() {
        Command::new(&*POST_EXEC_SCRIPT)
            .output()
            .map_err(|e| ExecError(e.to_string()))?;
    }

    Ok(())
}

fn build_selected_blueprints(args: &ArgMatches, colors: &mut ColorTable) -> Result<(), Error> {
    let blueprints = args.get_many::<String>("blueprint");

    // No specified blueprints. Builds 'em all
    if blueprints.is_none() {
        return build_blueprints(colors);
    }

    // Search and build each blueprint
    for blueprint in blueprints.unwrap() {
        let path = search_blueprint(blueprint).map_err(|e| SystemError(e))?;

        let result = build_blueprint(&path, colors);
        if let Err(BlueprintError(e)) = result {
            log_as_error(BlueprintError(format!(
                "While parsing blueprint `{}`. {}",
                path.display(),
                e
            )));
        }
    }

    Ok(())
}

fn search_blueprint(blueprint: &String) -> Result<PathBuf, String> {
    let paths = [
        CONFIG_BLUEPRINTS_DIR.join(blueprint),
        CACHE_BLUEPRINTS_DIR.join(blueprint),
        PathBuf::from(blueprint),
    ];

    for path in &paths {
        if path.exists() {
            return Ok(path.clone());
        }
    }

    Err(format!(
        "Can't find blueprint `{}`. Neither of the following files exists `{}`",
        blueprint,
        paths
            .iter()
            .map(|p| p.to_string_lossy())
            .collect::<Vec<_>>()
            .join("`, `")
    ))
}
