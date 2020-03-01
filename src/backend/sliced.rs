use crate::{
    backend::Backend,
    event::Event,
    style::AnsiStyle as Style,
    vec2::Vec2,
};

use std::time::Duration;

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
    fn print_at(
        &mut self,
        pos: Vec2,
        text: &str,
    ) {
        if pos.y < self.1.y {
            return;
        }

        if pos.x >= self.1.x {
            self.0.print_at(pos - self.1, text);
        }

        let (_, sub_str, left) =
            crate::util::slice_str_with_width(text, (self.1.x - pos.x) as usize);

        if !sub_str.is_empty() {
            self.0
                .print_at((left as u16, pos.y - self.1.y).into(), sub_str);
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    fn size(&self) -> Vec2 {
        self.0.size() + self.1
    }

    #[inline]
    fn flush(&mut self) {
        self.0.flush();
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
}
