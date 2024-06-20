use crate::config::environ::HOME_DIR;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn read_file<'a>(path: &PathBuf) -> Result<String, String> {
    // Opens the file
    let mut file = File::open(path.clone()).map_err(|e| e.to_string())?;

    // Reads its content
    let mut content = String::new();
    let _ = file
        .read_to_string(&mut content)
        .map_err(|e| e.to_string())?;

    Ok(content)
}

pub fn expand_home_dir(path: &str) -> PathBuf {
    if !path.starts_with("~/") {
        return PathBuf::from(path);
    }

    let mut expanded_path = PathBuf::from(&*HOME_DIR);
    expanded_path.push(&path[2..]);

    expanded_path
}
