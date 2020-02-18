use crate::modifires::Modifiers;
use crossterm::style::Color;
use enumflags2::BitFlags;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub fg:       Color,
    pub bg:       Color,
    pub modifier: BitFlags<Modifiers>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg:       Color::Reset,
            bg:       Color::Reset,
            modifier: BitFlags::empty(),
        }
    }
}

#[derive(Default)]
pub struct StyledText {
    spans: Vec<(String, Style)>,
    width: usize,
}

impl StyledText {
    pub fn new() -> Self {
        Self {
            spans: Vec::new(),
            width: 0,
        }
    }

    pub fn styled(
        text: String,
        style: Style,
    ) -> Self {
        let width = text.width();
        Self {
            spans: vec![(text, style)],
            width,
        }
    }

    pub fn append(
        &mut self,
        text: String,
        style: Style,
    ) {
        self.width += text.width();
        self.spans.push((text, style));
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn spans(&self) -> &[(String, Style)] {
        self.spans.as_slice()
    }
}
