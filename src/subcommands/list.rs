use crate::colortable::rgb::RGB;
use crate::colortable::ColorTable;
use crate::config::colorscheme;
use crate::config::environ::COLORSCHEMES_DIR;
use crate::logging::{log_as_error, Error, Error::SystemError};
use clap::ArgMatches;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

struct ColorschemeInfo {
    // path: PathBuf,
    name: String,
    background: RGB,
    foreground: RGB,
    background_luminance: f32,
}

impl ColorschemeInfo {
    fn new(path: &Path) -> Result<Self, String> {
        // Extracts the file name
        let filename = path
            .file_stem()
            .ok_or(format!("Can't extract file name of `{}`", path.display()))?
            .to_str()
            .ok_or(format!(
                "Can't convert filename of `{}` to str",
                path.display()
            ))?
            .to_string();

        // Loads the colorscheme
        let colortable = ColorTable::from_file_path(&path.to_path_buf())?;

        // Retrieves the bg/fg colors
        // FIX: prone to error. When updating crate::config::colorscheme::COLOR_NAMES,
        //      one has to remember to also update these two lines.
        let background = colortable.get("background").unwrap().clone();
        let foreground = colortable.get("foreground").unwrap().clone();

        // Evaluates the luminance
        let background_luminance = background.luminance();

        Ok(Self {
            // path: path.to_path_buf(),
            name: filename,
            background,
            foreground,
            background_luminance,
        })
    }
}

pub fn list(args: &ArgMatches) -> Result<(), Error> {
    // Reads files contained in COLORSCHEMES_DIR, than converts them
    // into ColorschemeInfo structs
    let mut colorscheme_infos: Vec<_> = read_dir(&*COLORSCHEMES_DIR)
        .map_err(|e| SystemError(e.to_string()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .map(|path| ColorschemeInfo::new(&path))
        .filter_map(Result::ok)
        .collect();

    // Finds the length of the longest colorscheme name
    let max_len: usize = colorscheme_infos
        .iter()
        .map(|c| c.name.len())
        .max()
        .unwrap();

    // Sorts them in alphabetical order
    colorscheme_infos.sort_by(|a, b| a.name.cmp(&b.name));

    for colorscheme in colorscheme_infos {
        let len_diff = max_len - colorscheme.name.len();
        let extra_spaces: String = std::iter::repeat(' ').take(len_diff as usize).collect();

        let _ = write_with_custom_colors(
            &colorscheme.background,
            &colorscheme.foreground,
            format!(" {}{} \n", colorscheme.name, extra_spaces),
        );
    }

    Ok(())
}

use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn write_with_custom_colors(bg: &RGB, fg: &RGB, text: String) -> io::Result<()> {
    // Creates the color spec
    let fg = Color::Rgb(fg.0, fg.1, fg.2);
    let bg = Color::Rgb(bg.0, bg.1, bg.2);

    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(fg));
    color_spec.set_bg(Some(bg));

    // Creates a new StandardStream and sets the color specification
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(&color_spec)?;

    // Prints the colored text
    write!(&mut stdout, "{}", text)?;

    // Reset the color to default
    stdout.reset()?;

    Ok(())
}
