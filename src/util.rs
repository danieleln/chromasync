use std::fs::DirEntry;
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

// Checks if a DirEntry struct is a file
pub fn is_file(entry: &DirEntry) -> bool {
    if let Ok(metadata) = entry.metadata() {
        return metadata.is_file();
    }

    false
}
