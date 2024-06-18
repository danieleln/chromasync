use super::info;
use crate::logging::{Error, Error::InvalidEnvironment};
use const_format::formatcp;
use once_cell::sync::Lazy;
use std::env;
use std::path::PathBuf;
use std::process;

// Home directory
pub static HOME_DIR: Lazy<PathBuf> = Lazy::new(|| match env::var("HOME") {
    Ok(home) => PathBuf::from(home),
    Err(_) => panic!("Can't find the $HOME directory!"),
});

//////////////////////
// Config directory //
//////////////////////
pub static CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| match env::var("XDG_CONFIG_HOME") {
    Ok(config) => PathBuf::from(config),
    Err(_) => HOME_DIR.join(".config").join(info::APP_NAME),
});

// Colorschemes directory
pub static COLORSCHEMES_DIR: Lazy<PathBuf> = Lazy::new(|| CONFIG_DIR.join("colorschemes"));

// Blueprints directory (inside CONFIG_DIR) -> managed by the user
pub static CONFIG_BLUEPRINTS_DIR: Lazy<PathBuf> = Lazy::new(|| CONFIG_DIR.join("blueprints"));

// Script that runs after generating all blueprints
pub static POST_EXEC_SCRIPT: Lazy<PathBuf> =
    Lazy::new(|| CONFIG_DIR.join(formatcp!("{}-post.sh", super::info::APP_NAME)));

/////////////////////
// Cache directory //
/////////////////////
pub static CACHE_DIR: Lazy<PathBuf> = Lazy::new(|| match env::var("XDG_CACHE_HOME") {
    Ok(cache) => PathBuf::from(cache),
    Err(_) => HOME_DIR.join(".cache").join(info::APP_NAME),
});

// Out directory
pub static OUT_DIR: Lazy<PathBuf> = Lazy::new(|| CACHE_DIR.join("out"));

// Blueprints directory (inside CACHE_DIR) -> managed by other plugins
pub static CACHE_BLUEPRINTS_DIR: Lazy<PathBuf> = Lazy::new(|| CACHE_DIR.join("blueprints"));

// Builds all directories
pub fn build_dirs() -> Result<(), Error> {
    use std::fs;

    fs::create_dir_all(&*CONFIG_DIR).map_err(|e| {
        InvalidEnvironment(format!("Can't create `{}` directory", CONFIG_DIR.display()))
    })?;

    fs::create_dir_all(&*COLORSCHEMES_DIR).map_err(|e| {
        InvalidEnvironment(format!(
            "Can't create the `{}` directory",
            COLORSCHEMES_DIR.display()
        ))
    })?;

    fs::create_dir_all(&*CONFIG_BLUEPRINTS_DIR).map_err(|e| {
        InvalidEnvironment(format!(
            "Can't create the `{}` directory",
            CONFIG_BLUEPRINTS_DIR.display()
        ))
    })?;

    fs::create_dir_all(&*CACHE_DIR).map_err(|e| {
        InvalidEnvironment(format!(
            "Can't create the `{}` directory",
            CACHE_DIR.display()
        ))
    })?;

    fs::create_dir_all(&*OUT_DIR).map_err(|e| {
        InvalidEnvironment(format!(
            "Can't create the `{}` directory",
            OUT_DIR.display()
        ))
    })?;

    fs::create_dir_all(&*CACHE_BLUEPRINTS_DIR).map_err(|e| {
        InvalidEnvironment(format!(
            "Can't create the `{}` directory",
            CACHE_BLUEPRINTS_DIR.display()
        ))
    })?;

    Ok(())
}
