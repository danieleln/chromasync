mod rgb;
mod visitor;

use rgb::RGB;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

// Color table that holds all the colors of the colorscheme plus
// OS-specific colors and mixed colors (weighted average of two colors)
#[derive(Debug)]
pub struct ColorTable(HashMap<String, RGB>);

impl ColorTable {
    pub fn with_capacity(capacity: usize) -> ColorTable {
        ColorTable(HashMap::with_capacity(capacity))
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
