use crate::blueprint::build_blueprints;
use crate::colortable::ColorTable;
use crate::config::colorscheme::FILE_EXTENSION;
use crate::config::environ::COLORSCHEMES_DIR;
use crate::logging::Error;
use clap::ArgMatches;

pub fn load(args: &ArgMatches) -> Result<(), Error> {
    let colorscheme_name = args.get_one::<String>("colorscheme").unwrap();

    // Complete file path
    let path = (&*COLORSCHEMES_DIR).join(format!("{}.{}", colorscheme_name, FILE_EXTENSION));
    let mut colors = ColorTable::from_file_path(&path).map_err(|e| Error::ColorschemeError(e))?;

    build_blueprints(&mut colors)?;

    Ok(())
}
