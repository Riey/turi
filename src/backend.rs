use crate::vec2::Vec2;
use ansi_term::Style;

#[cfg(feature = "crossterm-backend")]
mod crossterm;

#[cfg(feature = "crossterm-backend")]
pub use self::crossterm::{
    CrosstermBackend,
    CrosstermBackendGuard,
};

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
