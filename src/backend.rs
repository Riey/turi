use crate::{
    event::Event,
    vec2::Vec2,
};
use ansi_term::Style;
use std::time::Duration;

//#[cfg(feature = "crossterm-backend")]
mod crossterm;

mod dummy;

mod test;

mod sliced;

#[cfg(feature = "crossterm-backend")]
pub use self::crossterm::{
    CrosstermBackend,
    CrosstermBackendGuard,
};

#[cfg(feature = "test-backend")]
pub use self::test::TestBackend;
pub use self::{
    dummy::DummyBackend,
    sliced::SlicedBackend,
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
