// Themeing for the website

use ratzilla::ratatui::style::Color;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

//TODO: make the color palette catpuccin mocha  and catpuccin latte.
impl Theme {
    pub fn toggle(self) -> Self {
        match self {
            Self::Dark => Self::Light, // basically if checkign for current theme and then returning opposite.
            Self::Light => Self::Dark, // syntactically simpler version.
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Dark => "dark", //same here
            Self::Light => "light",
        }
    }

    pub fn palette(self) -> Palette {
        match self {
            Self::Dark => Palette {
                background: Color::Rgb(11, 15, 17),
                background_panel: Color::Rgb(16, 24, 39),
                background_life: Color::Rgb(70, 82, 89),
                foreground: Color::Rgb(217, 231, 229),
                muted: Color::Rgb(143, 161, 166),
                border: Color::Rgb(61, 76, 80),
                accent: Color::Rgb(107, 208, 227),
                warning: Color::Rgb(244, 191, 117),
                success: Color::Rgb(158, 206, 106),
                selection_background: Color::Rgb(25, 55, 64),
                selection_foreground: Color::Rgb(235, 253, 255),
            },
            Self::Light => Palette {
                background: Color::Rgb(244, 247, 248),
                background_panel: Color::Rgb(234, 241, 243),
                background_life: Color::Rgb(174, 192, 197),
                foreground: Color::Rgb(23, 33, 38),
                muted: Color::Rgb(83, 103, 109),
                border: Color::Rgb(174, 192, 197),
                accent: Color::Rgb(17, 114, 133),
                warning: Color::Rgb(158, 91, 28),
                success: Color::Rgb(45, 119, 68),
                selection_background: Color::Rgb(211, 237, 242),
                selection_foreground: Color::Rgb(13, 52, 61),
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub background: Color,
    pub background_panel: Color,
    pub background_life: Color,
    pub foreground: Color,
    pub muted: Color,
    pub border: Color,
    pub accent: Color,
    pub warning: Color,
    pub success: Color,
    pub selection_background: Color,
    pub selection_foreground: Color,
}
