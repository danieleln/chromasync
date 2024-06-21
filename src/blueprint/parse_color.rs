use super::parse_directive::Directive;
use crate::colortable::rgb::RGB;
use crate::colortable::ColorTable;
use crate::config::blueprint::MIXED_COLOR_FIELD_SEPARATOR;
use crate::logging::{log_as_warning, Error::BlueprintError};
use const_format::formatcp;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::path::PathBuf;

// Assuming MIXED_COLOR_FIELD_SEPARATOR = ":", the regex becomes r"\{(\w+)(:(\d+):(\w+))?\}"
static COLOR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(formatcp!(
        r"\{{(\w+)({MIXED_COLOR_FIELD_SEPARATOR}(\d+){MIXED_COLOR_FIELD_SEPARATOR}(\w+))?\}}"
    ))
    .unwrap()
});

pub fn parse_color(
    line: &str,
    colors: &mut ColorTable,
    directives: &Directive,
    blueprint: &PathBuf,
) -> String {
    COLOR_REGEX
        .replace_all(line, |caps: &Captures| {
            // Extracts colors from the caps groups
            let color1 = caps.get(1);
            let amount = caps.get(3);
            let color2 = caps.get(4);

            // The first color is always present
            let color1 = color1.unwrap().as_str().to_string();

            let color: Option<&RGB> = if color2.is_some() {
                // Returns the mixed color
                let color2: String = color2.unwrap().as_str().to_string();
                let amount: u8 = amount.unwrap().as_str().parse().unwrap();
                colors.get_composite(&color1, amount, &color2)
            } else {
                // Returns just the first color
                colors.get(&color1.to_string())
            };

            // The whole matching color expression (minus the starting/closing
            // curly braces). Used only when raising the last two errors
            let whole_match = caps.get(0).unwrap().as_str();
            let whole_color = &whole_match[1..whole_match.len() - 1];

            // Formats and returns the color
            if let Some(color) = color {
                let formatted_color = color.format(&directives.color_format);

                if formatted_color.is_ok() {
                    return formatted_color.unwrap();
                } else {
                    log_as_warning(BlueprintError(format!(
                        "While parsing blueprint `{}`. An error occurred while formatting color `{}` as `{}`. Can't replace it in the blueprint.",
                        blueprint.display(),
                        whole_color,
                        &directives.color_format
                    )));
                }
            } else {
                log_as_warning(BlueprintError(format!(
                    "While parsing blueprint `{}`. An error occurred while retrieving color `{}`. Can't replace it in the blueprint.",
                    blueprint.display(),
                    whole_color,
                )));
            }

            "".to_string()
        })
        .to_string()
}
