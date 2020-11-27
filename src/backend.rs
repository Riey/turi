use crate::vec2::Vec2;
use ansi_term::Style;

#[cfg(feature = "crossterm-backend")]
mod crossterm;

#[cfg(feature = "wgpu-backend")]
mod wgpu;

mod dummy;

#[cfg(feature = "buffer-backend")]
mod buffer;

mod sliced;

#[cfg(feature = "crossterm-backend")]
pub use self::crossterm::{
    CrosstermBackend,
    CrosstermBackendGuard,
};

#[cfg(feature = "wgpu-backend")]
pub use self::wgpu::WgpuBackend;

#[cfg(feature = "buffer-backend")]
pub use self::buffer::BufferBackend;
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
