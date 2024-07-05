use crate::colortable::rgb::RGB;
use crate::colortable::ColorTable;
use crate::config::colorscheme;
use crate::config::environ::COLORSCHEMES_DIR;
use crate::logging::{Error, Error::SystemError};
use crate::util::print_with_custom_colors;
use clap::ArgMatches;
use std::fs::read_dir;
use std::path::Path;

struct ColorschemeInfo {
    name: String,
    background: RGB,
    foreground: RGB,
    background_luminance: f32,
    contrast: f32,
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
        let background = colortable.get(colorscheme::BACKGROUND).unwrap().clone();
        let foreground = colortable.get(colorscheme::FOREGROUND).unwrap().clone();

        // Evaluates luminance and contrast
        let background_luminance = background.luminance();
        let foreground_luminance = foreground.luminance();
        let contrast = (foreground_luminance - background_luminance).abs();

        Ok(Self {
            name: filename,
            background,
            foreground,
            background_luminance,
            contrast,
        })
    }
}

pub fn list(args: &ArgMatches) -> Result<(), Error> {
    // Lists only dark/light theme
    let dark_only: bool = *args.get_one::<bool>("dark").ok_or(false).unwrap();
    let light_only: bool = *args.get_one::<bool>("light").ok_or(false).unwrap();

    // Reads files contained in COLORSCHEMES_DIR and converts them
    // into ColorschemeInfo structs
    let mut colorscheme_infos: Vec<_> = read_dir(&*COLORSCHEMES_DIR)
        .map_err(|e| SystemError(e.to_string()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        // Keeps only files
        .filter(|path| path.is_file())
        // Converts to ColorschemeInfo structs
        .map(|path| ColorschemeInfo::new(&path))
        .filter_map(Result::ok)
        // Filters based on the luminance
        .filter(|c| {
            if (dark_only && c.background_luminance >= 0.5)
                || (light_only && c.background_luminance < 0.5)
            {
                false
            } else {
                true
            }
        })
        .collect();

    // Sorts colorschemes according to the --sort-by flag
    let default_ordering = "name".to_string();
    let sort_by = args
        .get_one::<String>("sort-by")
        .or(Some(&default_ordering))
        .unwrap();
    match sort_by.as_str() {
        "name" | "n" => colorscheme_infos.sort_by(|a, b| a.name.cmp(&b.name)),
        "background_luminance"
        | "bg-lum"
        | "bg_lum"
        | "bglum"
        | "background"
        | "bg"
        | "luminance"
        | "lum"
        | "background-brightness"
        | "background_brightness"
        | "brightness" => colorscheme_infos.sort_by(|a, b| {
            a.background_luminance
                .partial_cmp(&b.background_luminance)
                .unwrap()
        }),
        "contrast" | "contr" | "cont" | "con" | "cntr" | "cnt" => {
            colorscheme_infos.sort_by(|a, b| {
                a.contrast //
                    .partial_cmp(&b.contrast)
                    .unwrap()
            })
        }
        _ => unreachable!(),
    }

    // Finds the length of the longest colorscheme name
    let max_len: usize = colorscheme_infos
        .iter()
        .map(|c| c.name.chars().count())
        .max()
        .unwrap();

    // Prints the headline
    println!(
        "│ {:<width$} │ {:<8} │ {:<4} │",
        "NAME",
        "LUM (BG)",
        "CONT",
        width = max_len
    );

    // Prints the colorschemes
    for colorscheme in colorscheme_infos {
        let _ = print_with_custom_colors(
            &colorscheme.background,
            &colorscheme.foreground,
            format!(
                "│ {:<width$} │ {:<8.2} │ {:<4.2} │\n",
                colorscheme.name,
                colorscheme.background_luminance,
                colorscheme.contrast,
                width = max_len,
            ),
        );
    }

    Ok(())
}
