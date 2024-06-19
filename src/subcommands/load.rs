use crate::blueprint::build_blueprints;
use crate::colortable::ColorTable;
use crate::config::colorscheme::FILE_EXTENSION;
use crate::config::environ::COLORSCHEMES_DIR;
use crate::logging::Error;
use crate::util;
use clap::ArgMatches;

pub fn load(args: &ArgMatches) -> Result<(), Error> {
    let colorscheme_name = args.get_one::<String>("colorscheme").unwrap();
    let mut colors = load_colorscheme_file(colorscheme_name)?;

    build_blueprints(&mut colors, &args)?;

    Ok(())
}

fn load_colorscheme_file(filename: &str) -> Result<ColorTable, Error> {
    let path = (&*COLORSCHEMES_DIR).join(format!("{}.{}", filename, FILE_EXTENSION));

    // Checks if the colorscheme file exists
    if !path.exists() {
        return Err(Error::ColorschemeError(format!(
            "Can't find colorscheme `{}`. Unable to locate the file `{}`",
            filename,
            path.display()
        )));
    }

    // Reads the file content
    let json_str = util::read_file(&path).map_err(|e| Error::ColorschemeError(e))?;

    // Parses the colorscheme
    let colorscheme = ColorTable::from_json_str(&json_str)?;

    Ok(colorscheme)
}
