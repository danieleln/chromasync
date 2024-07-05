use crate::colortable::ColorTable;
use crate::config::colorscheme::FILE_EXTENSION;
use crate::config::environ::COLORSCHEMES_DIR;
use crate::logging::Error;
use clap::ArgMatches;

pub fn preview(args: &ArgMatches) -> Result<(), Error> {
    // Complete file path
    let colorscheme_name = args.get_one::<String>("colorscheme").unwrap();
    let path = (&*COLORSCHEMES_DIR).join(format!("{}.{}", colorscheme_name, FILE_EXTENSION));

    // Loads the colorscheme
    let colors = ColorTable::from_file_path(&path).map_err(|e| Error::ColorschemeError(e))?;

    Ok(())
}
