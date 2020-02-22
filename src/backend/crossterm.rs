use crate::vec2::Vec2;
use ansi_term::Style;
use crossterm::{
    cursor::{
        Hide,
        MoveTo,
        Show,
    },
    event::{
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode,
        KeyEvent,
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
};
use std::{
    convert::TryFrom,
    io::Write,
};

use crate::{
    backend::Backend,
    events,
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

impl TryFrom<Event> for events::ClickEvent {
    type Error = Event;

    fn try_from(e: Event) -> Result<Self, Self::Error> {
        match e {
            Event::Mouse(MouseEvent::Down(..)) => Ok(Self),
            _ => Err(e),
        }
    }
}

impl TryFrom<Event> for events::CharEvent {
    type Error = Event;

    fn try_from(e: Event) -> Result<Self, Self::Error> {
        match e {
            Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                modifiers,
            }) if modifiers.is_empty() => Ok(Self(ch)),
            _ => Err(e),
        }
    }
}

impl TryFrom<Event> for events::EnterEvent {
    type Error = Event;

    fn try_from(e: Event) -> Result<Self, Self::Error> {
        match e {
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers,
            }) if modifiers.is_empty() => Ok(Self),
            _ => Err(e),
        }
    }
}

impl TryFrom<Event> for events::BackspaceEvent {
    type Error = Event;

    fn try_from(e: Event) -> Result<Self, Self::Error> {
        match e {
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                modifiers,
            }) if modifiers.is_empty() => Ok(Self),
            _ => Err(e),
        }
    }
}
