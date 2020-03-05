use crate::{
    backend::{
        Backend,
        SlicedBackend,
    },
    rect::Rect,
    style::Style,
    vec2::Vec2,
};
use std::mem::swap;

pub struct Printer<'a> {
    bound:   Rect,
    backend: &'a mut dyn Backend,
}

impl<'a> Printer<'a> {
    pub fn new(backend: &'a mut dyn Backend) -> Self {
        Self {
            bound: Rect::new((0, 0), backend.size()),
            backend,
        }
    }

    pub fn sliced<T>(
        &mut self,
        pos: impl Into<Vec2>,
        f: impl FnOnce(&mut Printer) -> T,
    ) -> T {
        let pos = pos.into();
        let mut backend = SlicedBackend::new(self.backend, pos);
        let mut printer = Printer {
            bound:   Rect::new(
                self.bound.start().saturating_sub(pos),
                self.bound.size() + pos,
            ),
            backend: &mut backend,
        };
        f(&mut printer)
    }

    pub fn with_bound<T>(
        &mut self,
        mut bound: Rect,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        swap(&mut self.bound, &mut bound);
        let ret = f(self);
        swap(&mut self.bound, &mut bound);
        ret
    }

    pub fn with_style<T>(
        &mut self,
        style: Style,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        let old_style = self.backend.style();
        self.backend.set_style(style);
        let ret = f(self);
        self.backend.set_style(old_style);
        ret
    }

    #[inline]
    pub fn clear(&mut self) {
        self.backend.clear();
    }

    #[inline]
    pub fn bound(&self) -> Rect {
        self.bound
    }

    pub fn style(&self) -> Style {
        self.backend.style()
    }

    pub fn print(
        &mut self,
        start: impl Into<Vec2>,
        text: &str,
    ) {
        let start = start.into();

        if !self.bound.contains_inclusive(start + self.bound.start()) {
            return;
        }

        let sub_str =
            crate::util::slice_str_with_width(text, (self.bound.end().x - start.x) as usize).0;
        self.raw_print(start, sub_str);
    }

    fn raw_print(
        &mut self,
        start: impl Into<Vec2>,
        text: &str,
    ) {
        self.backend
            .print_at(self.bound.start() + start.into(), text);
    }

    #[inline]
    pub fn print_styled(
        &mut self,
        start: impl Into<Vec2>,
        style: Style,
        text: &str,
    ) {
        self.with_style(style, |printer| {
            printer.print(start, text);
        });
    }

    #[inline]
    pub fn print_vertical_line(
        &mut self,
        pos: u16,
    ) {
        self.print_vertical_line_at((pos, 0), self.bound.h() as usize);
    }

    #[inline]
    pub fn print_vertical_line_at(
        &mut self,
        start: impl Into<Vec2>,
        size: usize,
    ) {
        const VLINE_CHAR: &str = "│";
        let start = start.into();

        // TODO: check bound
        for i in 0..size as u16 {
            self.raw_print(start.add_y(i), VLINE_CHAR);
        }
    }

    #[inline]
    pub fn print_vertical_block_line_at(
        &mut self,
        start: impl Into<Vec2>,
        size: usize,
    ) {
        const BLOCK_CHAR: &str = "█";
        let start = start.into();

        // TODO: check bound
        for i in 0..size as u16 {
            self.raw_print(start.add_y(i), BLOCK_CHAR);
        }
    }

    #[inline]
    pub fn print_horizontal_line(
        &mut self,
        pos: u16,
    ) {
        self.print_horizontal_line_at((0, pos), self.bound().w() as usize);
    }

    #[inline]
    pub fn print_horizontal_line_at(
        &mut self,
        start: impl Into<Vec2>,
        size: usize,
    ) {
        static BAR_STRING: &str = "────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────";
        self.raw_print(start, &BAR_STRING[..size * "─".len()]);
    }

    #[inline]
    pub fn print_horizontal_block_line_at(
        &mut self,
        start: impl Into<Vec2>,
        size: usize,
    ) {
        static BLOCK_STRING: &str = "██████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████";
        self.raw_print(start, &BLOCK_STRING[..size * "█".len()]);
    }

    pub fn print_rect(&mut self) {
        self.print_horizontal_line(0);
        self.print_horizontal_line(self.bound.h() - 1);
        self.print_vertical_line(0);
        self.print_vertical_line(self.bound.w() - 1);

        const LEFT_TOP: &str = "┌";
        const RIGHT_TOP: &str = "┐";
        const LEFT_BOTTOM: &str = "└";
        const RIGHT_BOTTOM: &str = "┘";

        let start = self.bound.start();
        let end = self.bound.end();

        self.raw_print(start, LEFT_TOP);
        self.raw_print((end.x, start.y), RIGHT_TOP);
        self.raw_print((start.x, end.y), LEFT_BOTTOM);
        self.raw_print(end, RIGHT_BOTTOM);
    }
}
