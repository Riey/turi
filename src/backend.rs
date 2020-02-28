use crate::{
    style::{
        BasicColor,
        Effect,
    },
    vec2::Vec2,
};

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
    fn set_bg(
        &mut self,
        color: BasicColor,
    );
    fn set_fg(
        &mut self,
        color: BasicColor,
    );
    fn set_effect(
        &mut self,
        effect: Effect,
    );
    fn unset_effect(
        &mut self,
        effect: Effect,
    );
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
    fn set_bg(
        &mut self,
        color: BasicColor,
    ) {
        (**self).set_bg(color);
    }

    #[inline]
    fn set_fg(
        &mut self,
        color: BasicColor,
    ) {
        (**self).set_fg(color);
    }

    #[inline]
    fn set_effect(
        &mut self,
        effect: Effect,
    ) {
        (**self).set_effect(effect);
    }

    #[inline]
    fn unset_effect(
        &mut self,
        effect: Effect,
    ) {
        (**self).unset_effect(effect);
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
    fn print_at(
        &mut self,
        _pos: Vec2,
        _text: &str,
    ) {
    }

    #[inline]
    fn flush(&mut self) {}

    #[inline]
    fn set_bg(
        &mut self,
        _color: BasicColor,
    ) {
    }

    #[inline]
    fn set_fg(
        &mut self,
        _color: BasicColor,
    ) {
    }

    #[inline]
    fn set_effect(
        &mut self,
        _effect: Effect,
    ) {
    }

    #[inline]
    fn unset_effect(
        &mut self,
        _effect: Effect,
    ) {
    }
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

        let mut left = self.1.x - pos.x;
        for (i, ch) in text.char_indices() {
            let width = ch.width().unwrap_or(0);
            let width = width as u16;
            if left < width {
                self.0
                    .print_at(Vec2::new(left, pos.y - self.1.y), text.split_at(i).1);
                return;
            } else {
                left -= width;
            }
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
    fn set_bg(
        &mut self,
        color: BasicColor,
    ) {
        self.0.set_bg(color);
    }

    #[inline]
    fn set_fg(
        &mut self,
        color: BasicColor,
    ) {
        self.0.set_fg(color);
    }

    #[inline]
    fn set_effect(
        &mut self,
        effect: Effect,
    ) {
        self.0.set_effect(effect);
    }

    #[inline]
    fn unset_effect(
        &mut self,
        effect: Effect,
    ) {
        self.0.unset_effect(effect);
    }
}
