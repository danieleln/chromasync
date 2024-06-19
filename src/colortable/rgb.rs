use once_cell::sync::Lazy;
use regex::Regex;

// Regex that recognizes HEX colors (#123456)
static REGEX_HEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#?[0-9a-fA-F]{6}$").unwrap());

// RGB tuple
#[derive(Debug)]
pub struct RGB(u8, u8, u8);

impl RGB {
    // Converts an HEX color into an RGB tuple
    pub fn new_from_hex(hex: &str) -> Result<Self, String> {
        let hex = hex.trim();

        if !(&*REGEX_HEX).is_match(&hex) {
            return Err(format!("Invalid hex color `{}`", hex));
        }

        // Removes the starting hashtag
        let hex = if hex.starts_with("#") { &hex[1..] } else { hex };

        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

        Ok(Self(r, g, b))
    }

    // Merges two RGB by weighting the current one by amount/100 and
    // the second one by 1-amount/100
    pub fn mix(&self, amount: u8, color: &Self) -> Self {
        let percentage: f32 = amount as f32 / 100.0;

        let _mix = |x, y| (x as f32 * percentage + y as f32 * (1.0 - percentage)) as u8;

        Self(
            _mix(self.0, color.0),
            _mix(self.1, color.1),
            _mix(self.2, color.2),
        )
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_with_hashtag() {
        let rgb = RGB::new_from_hex("  #000000   ");
        assert!(rgb.is_ok());
    }

    #[test]
    fn trim_without_hashtag() {
        let rgb = RGB::new_from_hex("  000000   ");
        assert!(rgb.is_ok());
    }

    #[test]
    fn number_conversion() {
        let rgb = RGB::new_from_hex("#80ED99").unwrap();
        assert!(rgb.0 == 128);
        assert!(rgb.1 == 237);
        assert!(rgb.2 == 153);
    }

    #[test]
    fn wrong_char() {
        let rgb = RGB::new_from_hex("#00y000");
        assert!(rgb.is_err());
        let rgb = RGB::new_from_hex("00y000");
        assert!(rgb.is_err());
    }

    #[test]
    fn wrong_length() {
        let rgb = RGB::new_from_hex("#000");
        assert!(rgb.is_err());
    }
}
