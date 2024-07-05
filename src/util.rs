use crate::config::environ::HOME_DIR;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn read_file<'a>(path: &PathBuf) -> Result<String, String> {
    // Opens the file
    let mut file = File::open(path.clone()).map_err(|e| e.to_string())?;

    // Reads its content
    let mut content = String::new();
    let _ = file
        .read_to_string(&mut content)
        .map_err(|e| e.to_string())?;

    Ok(content)
}

pub fn expand_home_dir(path: &str) -> PathBuf {
    if !path.starts_with("~/") {
        return PathBuf::from(path);
    }

    let mut expanded_path = PathBuf::from(&*HOME_DIR);
    expanded_path.push(&path[2..]);

    expanded_path
}

use crate::colortable::rgb::RGB;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// Writes strings to the terminal using custom RGB colors as bg/fg
pub fn print_with_custom_colors(bg: &RGB, fg: &RGB, text: String) -> io::Result<()> {
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
