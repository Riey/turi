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
};
use std::io::Write;

use crate::{
    backend::Backend,
    event::EventLike,
};
use crossterm::event::MouseButton;

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
        log::trace!("Resize to {:?}", size);
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

impl EventLike for Event {
    fn try_mouse_down(&self) -> Option<Vec2> {
        match self {
            Event::Mouse(MouseEvent::Down(MouseButton::Left, x, y, ..)) => Some((*x, *y).into()),
            _ => None,
        }
    }

    fn try_mouse_up(&self) -> Option<Vec2> {
        match self {
            Event::Mouse(MouseEvent::Up(MouseButton::Left, x, y, ..)) => Some((*x, *y).into()),
            _ => None,
        }
    }

    fn try_drag(&self) -> Option<Vec2> {
        match self {
            Event::Mouse(MouseEvent::Drag(MouseButton::Left, x, y, ..)) => Some((*x, *y).into()),
            _ => None,
        }
    }

    fn try_mouse(&self) -> Option<Vec2> {
        match self {
            Event::Mouse(MouseEvent::Down(_, x, y, ..))
            | Event::Mouse(MouseEvent::Up(_, x, y, ..))
            | Event::Mouse(MouseEvent::Drag(_, x, y, ..))
            | Event::Mouse(MouseEvent::ScrollUp(x, y, ..))
            | Event::Mouse(MouseEvent::ScrollDown(x, y, ..)) => Some((*x, *y).into()),
            _ => None,
        }
    }

    fn try_char(&self) -> Option<char> {
        match self {
            Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                modifiers,
            }) if *modifiers == KeyModifiers::empty() => Some(*ch),
            _ => None,
        }
    }

    fn try_ctrl_char(&self) -> Option<char> {
        match self {
            Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(*ch),
            _ => None,
        }
    }

    fn try_enter(&self) -> bool {
        match self {
            Event::Key(ke) if ke.code == KeyCode::Enter => true,
            _ => false,
        }
    }

    fn try_up(&self) -> bool {
        match self {
            Event::Key(ke) if ke.code == KeyCode::Up => true,
            _ => false,
        }
    }

    fn try_down(&self) -> bool {
        match self {
            Event::Key(ke) if ke.code == KeyCode::Down => true,
            _ => false,
        }
    }

    fn try_left(&self) -> bool {
        match self {
            Event::Key(ke) if ke.code == KeyCode::Left => true,
            _ => false,
        }
    }

    fn try_right(&self) -> bool {
        match self {
            Event::Key(ke) if ke.code == KeyCode::Right => true,
            _ => false,
        }
    }

    fn try_backspace(&self) -> bool {
        match self {
            Event::Key(ke) if ke.code == KeyCode::Backspace => true,
            _ => false,
        }
    }

    fn try_tab(&self) -> bool {
        match self {
            Event::Key(ke) if ke.code == KeyCode::Tab => true,
            _ => false,
        }
    }
}
