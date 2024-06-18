#![allow(unused)]

/////////////////
// Info Module //
/////////////////
pub mod info {
    use const_format::formatcp;

    pub const APP_NAME: &str = "chromasync";
    pub const DESCRIPTION: &str = formatcp!("`{APP_NAME}` is a tool designed to automate the process of changing colorschemes for various terminal applications.
");
}