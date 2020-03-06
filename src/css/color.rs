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
            "red" => Some(AnsiColor::Red),
            "green" => Some(AnsiColor::Green),
            "blue" => Some(AnsiColor::Blue),
            "black" => Some(AnsiColor::Black),
            "white" => Some(AnsiColor::White),
            "purple" => Some(AnsiColor::Purple),
            "yellow" => Some(AnsiColor::Yellow),
            "cyan" => Some(AnsiColor::Cyan),
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
