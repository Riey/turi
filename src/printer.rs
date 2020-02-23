use crate::{
    backend::Backend,
    rect::Rect,
    vec2::Vec2,
};
use ansi_term::{
    ANSIString,
    Style,
};
use std::mem::replace;
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

    pub fn with_bound<T>(
        &mut self,
        bound: Rect,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        let old_bound = replace(&mut self.bound, bound);
        let ret = f(self);
        self.bound = old_bound;
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

    #[inline(always)]
    pub fn refresh(&mut self) {
        self.backend.flush();
    }

    #[inline(always)]
    pub fn bound(&self) -> Rect {
        self.bound
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.backend.clear();
    }

    pub fn print(
        &mut self,
        start: impl Into<Vec2>,
        text: &str,
    ) {
        //TODO: check bound
        self.raw_print(start.into(), text);
    }

    fn raw_print(
        &mut self,
        start: impl Into<Vec2>,
        text: &str,
    ) {
        self.backend
            .print_at(self.bound.start() + start.into(), text);
    }

    pub fn print_styled(
        &mut self,
        start: impl Into<Vec2>,
        text: &ANSIString,
    ) {
        // TODO: check bound
        self.with_style(*text.style_ref(), |printer| {
            printer.raw_print(start, text);
        });
    }

    pub fn print_vertical_line(
        &mut self,
        pos: u16,
    ) {
        self.print_vertical_line_at((pos, 0), self.bound.h() as usize);
    }

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

    pub fn print_horizontal_line(
        &mut self,
        pos: u16,
    ) {
        self.print_horizontal_line_at((0, pos), self.bound().w() as usize);
    }

    pub fn print_horizontal_line_at(
        &mut self,
        start: impl Into<Vec2>,
        size: usize,
    ) {
        static BAR_STRING: &str = "────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────";
        self.print(start, &BAR_STRING[..size * "─".len()]);
    }

    pub fn print_horizontal_block_line_at(
        &mut self,
        start: impl Into<Vec2>,
        size: usize,
    ) {
        static BLOCK_STRING: &str = "██████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████";
        self.print(start, &BLOCK_STRING[..size * "█".len()]);
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

        self.backend.print_at(start, LEFT_TOP);
        self.backend.print_at((end.x, start.y).into(), RIGHT_TOP);
        self.backend.print_at((start.x, end.y).into(), LEFT_BOTTOM);
        self.backend.print_at(end, RIGHT_BOTTOM);
    }
}
