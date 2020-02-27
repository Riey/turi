use crate::vec2::Vec2;
use ansi_term::Style;

#[cfg(feature = "crossterm-backend")]
mod crossterm;

#[cfg(feature = "crossterm-backend")]
pub use self::crossterm::{
    CrosstermBackend,
    CrosstermBackendGuard,
};
use unicode_width::UnicodeWidthChar;

pub trait Backend {
    fn clear(&mut self);
    fn size(&self) -> Vec2;
    fn set_style(
        &mut self,
        style: Style,
    );
    fn style(&self) -> Style;
    fn print_at(
        &mut self,
        pos: Vec2,
        text: &str,
    );
    fn flush(&mut self);
}

impl<'a, B: Backend> Backend for &'a mut B {
    #[inline]
    fn clear(&mut self) {
        (**self).clear();
    }

    #[inline]
    fn size(&self) -> Vec2 {
        (**self).size()
    }

    #[inline]
    fn print_at(
        &mut self,
        pos: Vec2,
        text: &str,
    ) {
        (**self).print_at(pos, text);
    }

    #[inline]
    fn flush(&mut self) {
        (**self).flush();
    }

    #[inline]
    fn set_style(
        &mut self,
        style: Style,
    ) {
        (**self).set_style(style);
    }

    #[inline]
    fn style(&self) -> Style {
        (**self).style()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DummyBackend;

impl Backend for DummyBackend {
    #[inline]
    fn clear(&mut self) {}

    #[inline]
    fn size(&self) -> Vec2 {
        Vec2::new(0, 0)
    }

    #[inline]
    fn set_style(
        &mut self,
        _style: Style,
    ) {
    }

    #[inline]
    fn style(&self) -> Style {
        Style::default()
    }

    #[inline]
    fn print_at(
        &mut self,
        _pos: Vec2,
        _text: &str,
    ) {
    }

    #[inline]
    fn flush(&mut self) {}
}

pub struct SlicedBackend<'a>(&'a mut dyn Backend, Vec2);

impl<'a> SlicedBackend<'a> {
    pub fn new(
        backend: &'a mut dyn Backend,
        pos: Vec2,
    ) -> Self {
        Self(backend, pos)
    }
}

impl<'a> Backend for SlicedBackend<'a> {
    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    fn size(&self) -> Vec2 {
        self.0.size() + self.1
    }

    #[inline]
    fn set_style(
        &mut self,
        style: Style,
    ) {
        self.0.set_style(style);
    }

    #[inline]
    fn style(&self) -> Style {
        self.0.style()
    }

    #[inline]
    fn print_at(
        &mut self,
        pos: Vec2,
        text: &str,
    ) {
        if pos.y < self.1.y {
            return;
        }

        let mut left = self.1.x;
        for (i, ch) in text.char_indices() {
            let width = ch.width().unwrap_or(0);
            let width = width as u16;
            if left < width {
                self.0.print_at(pos, text.split_at(i).1);
                return;
            } else {
                left -= width;
            }
        }
    }

    #[inline]
    fn flush(&mut self) {
        self.0.flush();
    }
}
