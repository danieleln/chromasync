use serde::Deserialize;

pub const FILE_EXTENSION: &str = "json";

// Json deserialization struct
#[derive(Debug, Clone, Deserialize)]
pub struct JsonDeserStruct {
    // Background and foreground colors
    pub color_bg: String,
    pub color_fg: String,

    // Cursor color
    pub color_cu: String,

    // Normal colors
    pub color_01: String,
    pub color_02: String,
    pub color_03: String,
    pub color_04: String,
    pub color_05: String,
    pub color_06: String,
    pub color_07: String,
    pub color_08: String,

    // Highlight (bright) colors
    pub color_09: String,
    pub color_10: String,
    pub color_11: String,
    pub color_12: String,
    pub color_13: String,
    pub color_14: String,
    pub color_15: String,
    pub color_16: String,
}
