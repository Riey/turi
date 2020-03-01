use ansi_term::Style;
use crossterm::{
    cursor::{
        Hide,
        Show,
    },
    event::{
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode,
        KeyEvent,
        KeyModifiers,
        MouseEvent,
    },
    execute,
    queue,
    style::Print,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    Command,
};
use std::{
    fmt,
    io::Write,
};

use crate::{
    backend::Backend,
    event::{
        KeyCode as TKeyCode,
        KeyEvent as TKeyEvent,
        KeyModifiers as TKeyModifiers,
        MouseButton as TMouseButton,
        MouseEvent as TMouseEvent,
    },
    vec2::Vec2,
};

#[derive(Clone, Copy)]
struct TuriMoveTo(pub Vec2);

impl fmt::Display for TuriMoveTo {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(formatter, "\x1B[{};{}H", self.0.y + 1, self.0.x + 1)
    }
}

impl Command for TuriMoveTo {
    type AnsiType = Self;

    fn ansi_code(&self) -> Self::AnsiType {
        *self
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> crossterm::Result<()> {
        crossterm::cursor::MoveTo(self.0.x, self.0.y).execute_winapi()
    }
}

pub struct CrosstermBackend<W: Write> {
    out:   W,
    size:  Vec2,
    style: Style,
}

impl<W: Write> CrosstermBackend<W> {
    pub fn new(
        out: W,
        size: Vec2,
    ) -> Self {
        Self {
            out,
            size,
            style: Style::new(),
        }
    }

    pub fn resize(
        &mut self,
        size: Vec2,
    ) {
        self.size = size;
    }

    pub fn out(&mut self) -> &mut W {
        &mut self.out
    }
}

impl<W: Write> Backend for CrosstermBackend<W> {
    fn clear(&mut self) {
        queue!(self.out, Clear(ClearType::All)).unwrap();
    }

    fn size(&self) -> Vec2 {
        self.size
    }

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(
        &mut self,
        style: Style,
    ) {
        let diff = self.style.infix(style);
        self.style = style;

        queue!(self.out, Print(diff)).unwrap();
    }

    fn print_at(
        &mut self,
        pos: Vec2,
        text: &str,
    ) {
        queue!(self.out, TuriMoveTo(pos), Print(text),).unwrap();
    }

    fn flush(&mut self) {
        self.out.flush().unwrap();
    }
}

impl From<MouseEvent> for TMouseEvent {
    #[inline]
    fn from(e: MouseEvent) -> Self {
        match e {
            MouseEvent::Down(btn, x, y, _) => TMouseEvent::Down(TMouseButton::Left, (x, y).into()),
            _ => todo!(),
        }
    }
}

impl From<KeyEvent> for TKeyEvent {
    #[inline]
    fn from(e: KeyEvent) -> Self {
        todo!()
    }
}

pub struct CrosstermBackendGuard<W: Write> {
    inner: CrosstermBackend<W>,
}

impl<W: Write> Drop for CrosstermBackendGuard<W> {
    fn drop(&mut self) {
        execute!(
            self.inner.out(),
            Show,
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .ok();
        disable_raw_mode().ok();
    }
}

impl<W: Write> CrosstermBackendGuard<W> {
    pub fn new(mut inner: CrosstermBackend<W>) -> Self {
        enable_raw_mode().ok();
        execute!(inner.out(), Hide, EnterAlternateScreen, EnableMouseCapture).ok();

        Self { inner }
    }

    pub fn inner(&mut self) -> &mut CrosstermBackend<W> {
        &mut self.inner
    }
}
