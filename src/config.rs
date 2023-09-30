use penrose::Color;
use serde::{Deserialize, Serialize};

const DEFAULT_FONT: &'static str = "Sauce Code Pro Nerd Font";

#[derive(Debug, Serialize, Deserialize)]
pub struct WMConfig {
    pub colors: Colors,
    pub status_bar: StatusBar,
}

impl Default for WMConfig {
    fn default() -> Self {
        Self {
            colors: Colors::default(),
            status_bar: StatusBar::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontStyle {
    pub font: String,
    pub size: usize,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusBar {
    pub height: u32,
    pub font: FontStyle,
}

impl Default for StatusBar {
    fn default() -> Self {
        Self {
            height: 12,
            font: FontStyle {
                font: DEFAULT_FONT.to_string(),
                size: 8,
                fg: Color::new_from_hex(0xB5BFE2),
                bg: Color::new_from_hex(0x51576D),
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Colors {
    pub background: Color,
    pub foreground: Color,
    pub black: Color,
    pub dark_black: Color,
    pub red: Color,
    pub dark_red: Color,
    pub green: Color,
    pub dark_green: Color,
    pub yellow: Color,
    pub dark_yellow: Color,
    pub blue: Color,
    pub dark_blue: Color,
    pub magenta: Color,
    pub dark_magenta: Color,
    pub cyan: Color,
    pub dark_cyan: Color,
    pub white: Color,
    pub dark_white: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            background: Color::new_from_hex(0x303446),
            foreground: Color::new_from_hex(0xC6D0F5),
            black: Color::new_from_hex(0x51576D),
            dark_black: Color::new_from_hex(0x626880),
            red: Color::new_from_hex(0xE78284),
            dark_red: Color::new_from_hex(0xE78284),
            green: Color::new_from_hex(0xA6D189),
            dark_green: Color::new_from_hex(0xA6D189),
            yellow: Color::new_from_hex(0xE5C890),
            dark_yellow: Color::new_from_hex(0xE5C890),
            blue: Color::new_from_hex(0x8CAAEE),
            dark_blue: Color::new_from_hex(0x8CAAEE),
            magenta: Color::new_from_hex(0xF4B8E4),
            dark_magenta: Color::new_from_hex(0xF4B8E4),
            cyan: Color::new_from_hex(0x81C8BE),
            dark_cyan: Color::new_from_hex(0x81C8BE),
            white: Color::new_from_hex(0xB5BFE2),
            dark_white: Color::new_from_hex(0xA5ADCE),
        }
    }
}
