pub mod rgb;
mod visitor;

use crate::config::blueprint::MIXED_COLOR_FIELD_SEPARATOR;
use crate::util::read_file;
use rgb::RGB;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

// Color table that holds all the colors of the colorscheme plus
// OS-specific colors and mixed colors (weighted average of two colors)
#[derive(Debug)]
pub struct ColorTable(HashMap<String, RGB>);

impl ColorTable {
    pub fn with_capacity(capacity: usize) -> ColorTable {
        ColorTable(HashMap::with_capacity(capacity))
    }

    pub fn from_file_path(path: &PathBuf) -> Result<Self, String> {
        // Checks if the colorscheme file exists
        if !path.exists() {
            return Err(format!("Can't find colorscheme `{}`.", path.display()));
        }

        // Reads the file content
        let json_str = read_file(&path)?;

        // Parses the colorscheme
        serde_json::from_str(&json_str).map_err(|e| e.to_string())
    }

    pub fn get_composite(&mut self, color1: &String, amount: u8, color2: &String) -> Option<&RGB> {
        let name = format!(
            "{}{}{}{}{}",
            color1, MIXED_COLOR_FIELD_SEPARATOR, amount, MIXED_COLOR_FIELD_SEPARATOR, color2
        );

        if self.0.contains_key(&name) {
            return self.get(&name);
        }

        let composite = self._make_composite(color1, amount, color2);

        if composite.is_none() {
            return None;
        }

        self.0.insert(name.clone(), composite.unwrap());

        self.get(&name)
    }

    fn _make_composite(&self, color1: &String, amount: u8, color2: &String) -> Option<RGB> {
        let color1 = self.get(color1);
        let color2 = self.get(color2);

        if color1.is_none() || color2.is_none() {
            return None;
        }

        Some(color1.unwrap().mix(amount, color2.unwrap()))
    }
}

// Deref and DerefMut allows to access directly all the methods of the
// HashMap
impl Deref for ColorTable {
    type Target = HashMap<String, RGB>;

    fn deref(&self) -> &HashMap<String, RGB> {
        &self.0
    }
}

impl DerefMut for ColorTable {
    fn deref_mut(&mut self) -> &mut HashMap<String, RGB> {
        &mut self.0
    }
}
