use crate::{
    rect::Rect,
    style::{
        Style,
        StyledText,
    },
    vec2::Vec2,
};
use crossterm::{
    cursor::{
        Hide,
        MoveTo,
        Show,
    },
    event::{
        DisableMouseCapture,
        EnableMouseCapture,
    },
    execute,
    queue,
    style::{
        Print,
        SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::{
    io::Write,
    mem::replace,
};
use unicode_width::UnicodeWidthStr;

pub struct PrinterGuard<'a> {
    alternative: bool,
    out:         &'a mut dyn Write,
}

impl<'a> Drop for PrinterGuard<'a> {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.out, DisableMouseCapture, Show).unwrap();

        if self.alternative {
            execute!(self.out, LeaveAlternateScreen).unwrap()
        }
    }
}

impl<'a> PrinterGuard<'a> {
    pub fn new(
        out: &'a mut dyn Write,
        alternative: bool,
    ) -> Self {
        enable_raw_mode().unwrap();
        execute!(out, EnableMouseCapture, Hide).unwrap();

        if alternative {
            execute!(out, EnterAlternateScreen).unwrap()
        }

        Self { out, alternative }
    }

    #[inline(always)]
    pub fn make_printer(
        &mut self,
        size: impl Into<Vec2>,
    ) -> Printer<'_> {
        Printer::new(size, &mut self.out)
    }
}

pub struct Printer<'a> {
    bound: Rect,
    style: Style,
    out:   &'a mut dyn Write,
}

impl<'a> Printer<'a> {
    pub fn new(
        size: impl Into<Vec2>,
        out: &'a mut dyn Write,
    ) -> Self {
        Self {
            bound: Rect::new((0, 0), size),
            style: Style::default(),
            out,
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
        let old_style = replace(&mut self.style, style);
        let ret = f(self);
        self.style = old_style;
        ret
    }

    pub fn refresh(&mut self) {
        self.out.flush().unwrap();
    }

    #[inline(always)]
    pub fn bound(&self) -> Rect {
        self.bound
    }

    pub fn clear(&mut self) {
        queue!(
            self.out,
            SetBackgroundColor(self.style.bg),
            Clear(ClearType::All)
        )
        .unwrap();
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
        start: Vec2,
        text: &str,
    ) {
        let start = self.bound.start() + start;
        queue!(
            self.out,
            MoveTo(start.x, start.y),
            SetForegroundColor(self.style.fg),
            SetBackgroundColor(self.style.bg),
            Print(text)
        )
        .unwrap();
    }

    pub fn print_styled(
        &mut self,
        start: impl Into<Vec2>,
        text: &StyledText,
    ) {
        let mut start = start.into();
        // TODO: cut text when out of bound
        for span in text.spans() {
            let text = &span.0;
            self.style = span.1;
            self.raw_print(start, text);
            start.x += text.width() as u16;
        }
    }

    pub fn print_vertical_line(
        &mut self,
        pos: u16,
    ) {
        const VLINE_CHAR: &str = "│";

        let pos = self.bound.x() + pos;

        // TODO: check bound
        queue!(
            self.out,
            SetForegroundColor(self.style.fg),
            SetBackgroundColor(self.style.bg),
        )
        .unwrap();

        for i in 0..self.bound.h() {
            queue!(self.out, MoveTo(pos, self.bound.y() + i), Print(VLINE_CHAR),).unwrap();
        }
    }

    pub fn print_horizontal_line(
        &mut self,
        pos: u16,
    ) {
        const HLINE_STR: &str = "─";

        let size = self.bound.w();
        let pos = self.bound.y() + pos;
        let bar = HLINE_STR.repeat(size as usize);

        // TODO: check bound
        queue!(
            self.out,
            SetForegroundColor(self.style.fg),
            SetBackgroundColor(self.style.bg),
            MoveTo(self.bound.x(), pos),
            Print(bar),
        )
        .unwrap();
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

        queue!(
            self.out,
            MoveTo(start.x, start.y),
            Print(LEFT_TOP),
            MoveTo(end.x, start.y),
            Print(RIGHT_TOP),
            MoveTo(start.x, end.y),
            Print(LEFT_BOTTOM),
            MoveTo(end.x, end.y),
            Print(RIGHT_BOTTOM),
        )
        .unwrap();
    }
}
