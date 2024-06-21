use crate::blueprint::build_blueprints;
use crate::colortable::ColorTable;
use crate::config::blueprint::directive::HEX_6_DIGITS_W_HASHTAG;
use crate::config::colorscheme::FILE_EXTENSION;
use crate::config::environ::{COLORSCHEMES_DIR, CURRENT_COLORSCHEME_FILE};
use crate::logging::{Error, log_as_warning};
use clap::ArgMatches;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub fn load(args: &ArgMatches) -> Result<(), Error> {
    // Complete file path
    let colorscheme_name = args.get_one::<String>("colorscheme").unwrap();
    let path = (&*COLORSCHEMES_DIR).join(format!("{}.{}", colorscheme_name, FILE_EXTENSION));

    // Loads the colorscheme
    let mut colors = ColorTable::from_file_path(&path).map_err(|e| Error::ColorschemeError(e))?;

    // Stores a copy of the current colorscheme
    let result = backup_colorscheme(&colors).map_err(|e| {
        Error::SystemError(format!(
            "While storing a copy of the current colorscheme in `{}`. {}",
            CURRENT_COLORSCHEME_FILE.display(),
            e
        ))
    });

    if let Err(e) = result {
        log_as_warning(e);
    }

    // Instantiates all the blueprints
    build_blueprints(&mut colors)?;

    Ok(())
}

fn backup_colorscheme(colors: &ColorTable) -> Result<(), String> {
    // Replaces RGB colors with hexadecimal color strings
    let hex_colors: HashMap<String, String> = colors
        .iter()
        .map(|(name, rgb)| {
            (
                name.clone(),
                rgb.format(&HEX_6_DIGITS_W_HASHTAG.to_string()).unwrap(),
            )
        })
        .collect();

    // Converts the hash map to a json string
    let json_string = serde_json::to_string(&hex_colors).map_err(|e| e.to_string())?;

    // Stores the colorscheme
    let mut file = File::create(&*CURRENT_COLORSCHEME_FILE).map_err(|e| e.to_string())?;

    file.write_all(json_string.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(())
}
