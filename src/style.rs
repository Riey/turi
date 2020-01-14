use crate::modifires::Modifiers;
use crossterm::style::Color;
use enumflags2::BitFlags;

#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub modifier: BitFlags<Modifiers>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: BitFlags::empty(),
        }
    }
}
