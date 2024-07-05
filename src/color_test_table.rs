use crate::colortable::ColorTable;
use crate::config::colorscheme::{BACKGROUND, FOREGROUND};
use crate::config::preview::{color_alias, COL_COLOR_NAMES, ROW_COLOR_NAMES};
use crate::util::print_with_custom_colors;

pub fn print_color_test_table(colors: &ColorTable) {
    // Main background and foreground color
    let main_bg = colors.get(BACKGROUND).unwrap();
    let main_fg = colors.get(FOREGROUND).unwrap();

    // Prints the headline of the color test table (all the col color names)
    let _ = print_with_custom_colors(main_bg, main_fg, "   ".to_string());
    for col_color in &COL_COLOR_NAMES {
        let _ = print_with_custom_colors(
            main_bg,
            main_fg,
            format!("   {:<2} ", color_alias(col_color)),
        );
    }
    let _ = print_with_custom_colors(main_bg, main_fg, " \n".to_string());

    // Prints each row
    for row_color in &ROW_COLOR_NAMES {
        // Prints the color name at the beginning of the row
        let color_name = format!(" {:>2}", color_alias(&row_color));
        let _ = print_with_custom_colors(main_bg, main_fg, color_name);

        let row_color = row_color.to_string();

        // Prints each row
        for col_color in &COL_COLOR_NAMES {
            let col_color = col_color.to_string();

            let _ = print_with_custom_colors(main_bg, main_fg, " ".to_string());

            // Prints test string `gYw`
            let fg = colors.get(&row_color).unwrap();
            let bg = colors.get(&col_color).unwrap();
            let _ = print_with_custom_colors(bg, fg, " gYw ".to_string());
        }

        let _ = print_with_custom_colors(main_bg, main_fg, " \n".to_string());
    }

    // Empty line
    let _ = print_with_custom_colors(main_bg, main_fg, format!("{:<58}\n", ""));
}
