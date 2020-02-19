use crossterm::{
    execute,
    queue,
    style::Print,
    cursor::MoveTo,
    terminal::{Clear, ClearType},
};
use ansi_term::Style;
use std::io::Write;
use crate::vec2::Vec2;

use crate::backend::Backend;

pub struct CrosstermBackend<W: Write> {
    out: W,
    style: Style,
    size: Vec2,
}

impl<W: Write> Backend for CrosstermBackend<W> {
    fn clear(&mut self) {
        execute!(self.out, Clear(ClearType::All)).unwrap();
    }

    fn size(&self) -> Vec2 {
        self.size
    }

    fn set_style(
        &mut self,
        style: Style,
    ) {
        self.style = style;
    }

    fn style(&self) -> Style {
        self.style
    }

    fn print_at(
        &mut self,
        pos: Vec2,
        text: &str,
    ) {
        queue!(self.out, MoveTo(pos.x, pos.y), Print(self.style.paint(text))).unwrap();
    }

    fn flush(&mut self) {
        self.out.flush().unwrap();
    }
}
