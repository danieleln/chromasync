pub const MIXED_COLOR_FIELD_SEPARATOR: &str = ":";

pub mod directive {
    // Each directive consists of "{PREFIX}{DIRECTIVE}{SEPARATOR} {VALUE}"
    pub const PREFIX: &str = "%";
    pub const SEPARATOR: &str = " ";

    // DIRECTIVE
    pub const DIRECTIVE_COLOR_FORMAT: &str = "color-format";
    pub const DIRECTIVE_OUTPUT_DIRECTORY: &str = "output-directory";

    // Color formats
    pub const HEX_6_DIGITS_W_HASHTAG: &'static str = "#6h";
    pub const HEX_6_DIGITS_WO_HASHTAG: &'static str = "6h";
}
