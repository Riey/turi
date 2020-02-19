use crate::vec2::Vec2;
use ansi_term::Style;

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
    #[inline(always)]
    fn clear(&mut self) {
        (**self).clear();
    }

    #[inline(always)]
    fn size(&self) -> Vec2 {
        (**self).size()
    }

    #[inline(always)]
    fn print_at(
        &mut self,
        pos: Vec2,
        text: &str,
    ) {
        (**self).print_at(pos, text);
    }

    #[inline(always)]
    fn flush(&mut self) {
        (**self).flush();
    }

    #[inline(always)]
    fn set_style(
        &mut self,
        style: Style,
    ) {
        (**self).set_style(style);
    }

    #[inline(always)]
    fn style(&self) -> Style {
        (**self).style()
    }
}

