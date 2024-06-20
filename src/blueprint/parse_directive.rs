use crate::config::blueprint::directive::{
    DIRECTIVE_COLOR_FORMAT, DIRECTIVE_OUTPUT_DIRECTORY, HEX_6_DIGITS_WO_HASHTAG,
    HEX_6_DIGITS_W_HASHTAG, PREFIX, SEPARATOR,
};
use crate::config::environ::OUT_DIR;
use crate::util::expand_home_dir;
use const_format::formatcp;
use once_cell::sync::Lazy;
use regex::Regex;
use std::path::PathBuf;

static REGEX_KEY_VAL_DIRECTIVE: Lazy<Regex> =
    Lazy::new(|| Regex::new(formatcp!(r"^{PREFIX}((\w|-)+){SEPARATOR}\s*(.+)\s*$")).unwrap());

const COLOR_FORMATS: [&str; 2] = [HEX_6_DIGITS_WO_HASHTAG, HEX_6_DIGITS_W_HASHTAG];
const DIRECTIVE_TYPES: [&str; 2] = [DIRECTIVE_COLOR_FORMAT, DIRECTIVE_OUTPUT_DIRECTORY];

#[derive(Debug)]
pub struct Directive {
    pub color_format: String,
    pub output_directory: PathBuf,
}

impl Directive {
    // Creates a new Directive struct. The default output directory is
    // the parent of the blueprint being parsed
    pub fn new() -> Self {
        Self {
            color_format: HEX_6_DIGITS_W_HASHTAG.to_string(),
            output_directory: OUT_DIR.to_path_buf(),
        }
    }

    pub fn parse(&mut self, line: &str) -> Result<(), String> {
        // Matches the directive regex pattern against the line
        let caps = (&*REGEX_KEY_VAL_DIRECTIVE)
            .captures(line)
            .ok_or(format!("Ill formed directive `{}`.", line))?;

        // Groups 1 and 3 are the directive's type and value
        let directive_type = caps.get(1).unwrap().as_str();
        let directive_value = caps.get(3).unwrap().as_str();

        match directive_type {
            // Color format directive
            DIRECTIVE_COLOR_FORMAT => self.update_color_format(directive_value)?,

            // Output directory directive
            DIRECTIVE_OUTPUT_DIRECTORY => self.update_output_directory(directive_value)?,

            // Invalid directive
            _ => {
                return Err(format!(
                    "Invalid directive `{}`. Valid directives are `{}`",
                    directive_type,
                    DIRECTIVE_TYPES.join("`, `")
                ))
            }
        }

        Ok(())
    }

    fn update_color_format(&mut self, color_format: &str) -> Result<(), String> {
        // Checks if the color format exists
        if !COLOR_FORMATS.contains(&color_format) {
            return Err(format!(
                "Invalid color format `{}`. Valid color formats are `{}`.",
                color_format,
                COLOR_FORMATS.join("`, `")
            ));
        }

        // Updates self
        self.color_format = color_format.to_string();

        Ok(())
    }

    fn update_output_directory(&mut self, output_directory: &str) -> Result<(), String> {
        let output_directory = expand_home_dir(&output_directory);
        // Checks if the directory exists and if it's actually
        // a directory
        if !output_directory.exists() {
            return Err(format!(
                "Output directory `{}` doesn't exist.",
                output_directory.display(),
            ));
        }
        if !output_directory.is_dir() {
            return Err(format!(
                "Output directory `{}` is not a directory.",
                output_directory.display(),
            ));
        }

        // Updates self
        self.output_directory = output_directory.to_path_buf();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::blueprint::directive::{
        DIRECTIVE_COLOR_FORMAT, DIRECTIVE_OUTPUT_DIRECTORY, HEX_6_DIGITS_W_HASHTAG, PREFIX,
        SEPARATOR,
    };

    fn empty_directive() -> Directive {
        Directive {
            color_format: HEX_6_DIGITS_W_HASHTAG.to_string(),
            output_directory: PathBuf::from("/tmp"),
        }
    }

    #[test]
    fn color_format_directive() {
        let mut d = empty_directive();
        for color_format in COLOR_FORMATS {
            let statement =
                format!("{PREFIX}{DIRECTIVE_COLOR_FORMAT}{SEPARATOR}{color_format}").to_owned();
            let result = d.parse(&statement);
            assert!(
                result.is_ok(),
                "Directive `{}` failed to pass the test",
                statement
            );
            assert!(
                d.color_format == color_format,
                "Directive `{}`. Color format did not updated correctly",
                statement
            );
        }
    }

    #[test]
    fn output_directory_directive() {
        let out_dir = "/home";
        let mut d = empty_directive();
        let statement =
            format!("{PREFIX}{DIRECTIVE_OUTPUT_DIRECTORY}{SEPARATOR}{out_dir}").to_owned();
        let result = d.parse(&statement);
        assert!(
            result.is_ok(),
            "Directive `{}` failed to pass the test",
            statement
        );
        assert!(
            d.output_directory.to_str().unwrap() == out_dir,
            "Directive `{}`. Output directory did not updated correctly",
            statement
        );
    }
}
