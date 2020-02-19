use crate::vec2::Vec2;
use ansi_term::Style;
use crossterm::{
    cursor::MoveTo,
    event::{
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
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
};
use std::io::Write;

use crate::{
    backend::Backend,
    printer::Printer,
    view::View,
};

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

    pub fn out(&mut self) -> &mut W {
        &mut self.out
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

pub struct CrosstermBackendGuard<W: Write> {
    inner: CrosstermBackend<W>,
}

impl<W: Write> Drop for CrosstermBackendGuard<W> {
    fn drop(&mut self) {
        execute!(self.inner.out(), LeaveAlternateScreen, DisableMouseCapture).ok();
        disable_raw_mode().ok();
    }
}

impl<W: Write> CrosstermBackendGuard<W> {
    pub fn new(mut inner: CrosstermBackend<W>) -> Self {
        enable_raw_mode().ok();
        execute!(inner.out(), EnterAlternateScreen, EnableMouseCapture).ok();

        Self { inner }
    }

    pub fn inner(&mut self) -> &mut CrosstermBackend<W> {
        &mut self.inner
    }
}

pub fn crossterm_run<S, V: View<S, Event = Event, Message = Option<bool>>, W: Write>(
    state: &mut S,
    backend: &mut CrosstermBackend<W>,
    view: &mut V,
) {
    backend.clear();
    view.layout(backend.size());
    view.render(&mut Printer::new(backend));
    backend.flush();

    loop {
        match crossterm::event::read().unwrap() {
            Event::Resize(x, y) => {
                backend.resize((x, y).into());
            }
            e => {
                match view.on_event(state, e) {
                    Some(exit) => {
                        view.layout(backend.size());
                        view.render(&mut Printer::new(backend));
                        backend.flush();
                        if exit {
                            break;
                        }
                    }
                    None => continue,
                }
            }
        }
    }
}
