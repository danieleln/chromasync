use super::colorscheme::*;

pub const ROW_COLOR_NAMES: [&str; 18] = [
    FOREGROUND, CURSOR, COLOR_01, COLOR_09, COLOR_02, COLOR_10, COLOR_03, COLOR_11, COLOR_04,
    COLOR_12, COLOR_05, COLOR_13, COLOR_06, COLOR_14, COLOR_07, COLOR_15, COLOR_08, COLOR_16,
];

pub const COL_COLOR_NAMES: [&str; 9] = [
    BACKGROUND, COLOR_01, COLOR_02, COLOR_03, COLOR_04, COLOR_05, COLOR_06, COLOR_07, COLOR_08,
];

pub fn color_alias(color: &str) -> &str {
    match color {
        FOREGROUND => "f",
        BACKGROUND => "b",
        CURSOR => "c",
        COLOR_01 => "1",
        COLOR_02 => "2",
        COLOR_03 => "3",
        COLOR_04 => "4",
        COLOR_05 => "5",
        COLOR_06 => "6",
        COLOR_07 => "7",
        COLOR_08 => "8",
        COLOR_09 => "9",
        COLOR_10 => "10",
        COLOR_11 => "11",
        COLOR_12 => "12",
        COLOR_13 => "13",
        COLOR_14 => "14",
        COLOR_15 => "15",
        COLOR_16 => "16",
        _ => "?",
    }
}
