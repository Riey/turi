use crate::css::{
    combine::Combine,
    AnsiColor,
};
use css_color_parser::{
    Color,
    ColorParseError,
};
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub struct CssColor {
    color: Option<AnsiColor>,
}

impl CssColor {
    pub fn new(color: Option<AnsiColor>) -> Self {
        Self { color }
    }

    pub fn color(self) -> Option<AnsiColor> {
        self.color
    }
}

impl FromStr for CssColor {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color = match s {
            "transparent" => None,
            "black" => Some(AnsiColor::Black),
            "darkred" => Some(AnsiColor::Red),
            "darkmagenta" => Some(AnsiColor::Purple),
            "darkcyan" => Some(AnsiColor::Cyan),
            "darkgreen" => Some(AnsiColor::Green),
            "darkblue" => Some(AnsiColor::Blue),
            "lightgray" => Some(AnsiColor::White),
            "khaki" => Some(AnsiColor::Yellow),
            "gray" => Some(AnsiColor::Fixed(8)),
            "red" => Some(AnsiColor::Fixed(9)),
            "green" => Some(AnsiColor::Fixed(10)),
            "yellow" => Some(AnsiColor::Fixed(11)),
            "blue" => Some(AnsiColor::Fixed(12)),
            "magenta" => Some(AnsiColor::Fixed(13)),
            "cyan" => Some(AnsiColor::Fixed(14)),
            "white" => Some(AnsiColor::Fixed(15)),
            // TODO: other ansi256 color
            s => {
                match s.parse() {
                    Ok(Color { r, g, b, a: _ }) => Some(AnsiColor::RGB(r, g, b)),
                    Err(err) => return Err(err),
                }
            }
        };

        Ok(Self::new(color))
    }
}

impl Combine for CssColor {
    fn combine(
        self,
        other: Self,
    ) -> Self {
        other
    }
}
