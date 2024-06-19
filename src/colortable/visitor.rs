// Deserialization of a custom map.
// See https://serde.rs/deserialize-map.html for the detailed explanation

use super::rgb::RGB;
use super::ColorTable;
use crate::config::colorscheme::COLOR_NAMES;
use const_format::formatcp;
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use std::fmt;
use std::marker::PhantomData;

// A Visitor is a type that holds methods that a Deserializer can drive
// depending on what is contained in the input data.
//
// In the case of a map we need generic type parameters K and V to be
// able to set the output type correctly, but don't require any state.
// This is an example of a "zero sized type" in Rust. The PhantomData
// keeps the compiler from complaining about unused generic type
// parameters.
struct ColorTableVisitor {
    marker: PhantomData<fn() -> ColorTable>,
}

impl ColorTableVisitor {
    fn new() -> Self {
        ColorTableVisitor {
            marker: PhantomData,
        }
    }
}

// This is the trait that Deserializers are going to be driving. There
// is one method for each type of data that our type knows how to
// deserialize from. There are many other methods that are not
// implemented here, for example deserializing from integers or strings.
// By default those methods will return an error, which makes sense
// because we cannot deserialize a ColorTable from an integer or string.
impl<'de> Visitor<'de> for ColorTableVisitor {
    // The type that our Visitor is going to produce.
    type Value = ColorTable;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(formatcp!("a {} colorscheme", crate::config::info::APP_NAME))
    }

    // Deserialize ColorTable from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut colors = ColorTable::with_capacity(access.size_hint().unwrap_or(0));

        // Adds each entry into the ColorTable
        while let Some((color_name, color_hex)) = access.next_entry()? {
            // Checks whether color_name was already present
            if colors.contains_key(color_name) {
                return Err(de::Error::custom(format!(
                    "Color `{}` was already defined",
                    color_name
                )));
            }

            // Checks whether color_name is valid
            if !COLOR_NAMES.contains(&color_name) {
                // TODO: print which are the valid color names
                return Err(de::Error::custom(format!(
                    "Invalid color name `{}`",
                    color_name
                )));
            }

            // Converts the hex string into an RGB struct
            let color_rgb = RGB::new_from_hex(color_hex).map_err(|e| de::Error::custom(e))?;

            colors.insert(color_name.to_owned(), color_rgb);
        }

        // Checks whether there are missing colors:
        if colors.len() < COLOR_NAMES.len() {
            for &color_name in &COLOR_NAMES {
                if !colors.contains_key(color_name) {
                    return Err(de::Error::custom(format!(
                        "Missing required color `{}`",
                        color_name
                    )));
                }
            }
        }

        Ok(colors)
    }
}

// This is the trait that informs Serde how to deserialize ColorTable.
impl<'de> Deserialize<'de> for ColorTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of ColorTable.
        deserializer.deserialize_map(ColorTableVisitor::new())
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_correct_colorscheme() {
        let json = r##"{ "background": "#000000", "foreground": "#000000", "cursor": "#000000", "color_01": "#000000", "color_02": "#000000", "color_03": "#000000", "color_04": "#000000", "color_05": "#000000", "color_06": "#000000", "color_07": "#000000", "color_08": "#000000", "color_09": "#000000", "color_10": "#000000", "color_11": "#000000", "color_12": "#000000", "color_13": "#000000", "color_14": "#000000", "color_15": "#000000", "color_16": "#000000" }"##;
        let colorscheme = serde_json::from_str::<ColorTable>(json);
        assert!(colorscheme.is_ok());
    }

    #[test]
    fn test_missing_background() {
        let json = r##"{ "foreground": "#000000", "cursor": "#000000", "color_01": "#000000", "color_02": "#000000", "color_03": "#000000", "color_04": "#000000", "color_05": "#000000", "color_06": "#000000", "color_07": "#000000", "color_08": "#000000", "color_09": "#000000", "color_10": "#000000", "color_11": "#000000", "color_12": "#000000", "color_13": "#000000", "color_14": "#000000", "color_15": "#000000", "color_16": "#000000" }"##;
        let colorscheme = serde_json::from_str::<ColorTable>(json);
        assert!(colorscheme.is_err());
    }

    #[test]
    fn test_two_backgrounds() {
        let json = r##"{ "background": "#000000", "background": "#000000", "foreground": "#000000", "cursor": "#000000", "color_01": "#000000", "color_02": "#000000", "color_03": "#000000", "color_04": "#000000", "color_05": "#000000", "color_06": "#000000", "color_07": "#000000", "color_08": "#000000", "color_09": "#000000", "color_10": "#000000", "color_11": "#000000", "color_12": "#000000", "color_13": "#000000", "color_14": "#000000", "color_15": "#000000", "color_16": "#000000" }"##;
        let colorscheme = serde_json::from_str::<ColorTable>(json);
        assert!(colorscheme.is_err());
    }

    #[test]
    fn test_wrong_name() {
        let json = r##"{ "wrong_color_name": "#000000", "foreground": "#000000", "cursor": "#000000", "color_01": "#000000", "color_02": "#000000", "color_03": "#000000", "color_04": "#000000", "color_05": "#000000", "color_06": "#000000", "color_07": "#000000", "color_08": "#000000", "color_09": "#000000", "color_10": "#000000", "color_11": "#000000", "color_12": "#000000", "color_13": "#000000", "color_14": "#000000", "color_15": "#000000", "color_16": "#000000" }"##;
        let colorscheme = serde_json::from_str::<ColorTable>(json);
        assert!(colorscheme.is_err());
    }
}
