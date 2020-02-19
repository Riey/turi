use crate::vec2::Vec2;
use ansi_term::Style;
use crossterm::{
    cursor::MoveTo,
    execute,
    queue,
    style::Print,
    terminal::{
        Clear,
        ClearType,
    },
};
use std::io::Write;

use crate::backend::Backend;

pub struct CrosstermBackend<W: Write> {
    out:   W,
    style: Style,
    size:  Vec2,
}

impl<W: Write> CrosstermBackend<W> {
    pub fn new(
        out: W,
        size: Vec2,
    ) -> Self {
        Self {
            out,
            style: Style::default(),
            size,
        }
    }

    pub fn resize(
        &mut self,
        size: Vec2,
    ) {
        self.size = size;
    }
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
        queue!(
            self.out,
            MoveTo(pos.x, pos.y),
            Print(self.style.paint(text))
        )
        .unwrap();
    }

    fn flush(&mut self) {
        self.out.flush().unwrap();
    }
}
