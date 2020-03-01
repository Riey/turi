use crate::{
    backend::Backend,
    event::Event,
    style::AnsiStyle as Style,
    vec2::Vec2,
};

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
    fn print_at(
        &mut self,
        _pos: Vec2,
        _text: &str,
    ) {
    }

    #[inline]
    fn flush(&mut self) {}

    #[inline]
    fn set_style(
        &mut self,
        _style: Style,
    ) {
    }

    #[inline]
    fn style(&self) -> Style {
        Style::new()
    }
}
