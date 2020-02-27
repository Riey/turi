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
    event::{
        EventLike,
        KeyEventLike,
        MouseEventLike,
    },
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

macro_rules! code_is {
    ($event:expr, $code:ident) => {
        if let KeyEvent {
            code: KeyCode::$code,
            modifiers,
        } = $event
        {
            modifiers.is_empty()
        } else {
            false
        }
    };
}

impl KeyEventLike for KeyEvent {
    fn try_tab(&self) -> bool {
        code_is!(self, Tab)
    }

    fn try_up(&self) -> bool {
        code_is!(self, Up)
    }

    fn try_down(&self) -> bool {
        code_is!(self, Down)
    }

    fn try_left(&self) -> bool {
        code_is!(self, Left)
    }

    fn try_right(&self) -> bool {
        code_is!(self, Right)
    }

    fn try_char(&self) -> Option<char> {
        if let KeyEvent {
            code: KeyCode::Char(ch),
            modifiers,
        } = self
        {
            if modifiers.is_empty() {
                Some(*ch)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn try_ctrl_char(&self) -> Option<char> {
        if let KeyEvent {
            code: KeyCode::Char(ch),
            modifiers: KeyModifiers::CONTROL,
        } = self
        {
            Some(*ch)
        } else {
            None
        }
    }

    fn try_enter(&self) -> bool {
        code_is!(self, Enter)
    }

    fn try_backspace(&self) -> bool {
        code_is!(self, Backspace)
    }
}

impl MouseEventLike for MouseEvent {
    fn pos(&self) -> Vec2 {
        match self {
            MouseEvent::Down(_, x, y, ..)
            | MouseEvent::Up(_, x, y, ..)
            | MouseEvent::Drag(_, x, y, ..)
            | MouseEvent::ScrollUp(x, y, ..)
            | MouseEvent::ScrollDown(x, y, ..) => Vec2::new(*x, *y),
        }
    }

    fn map_pos(
        &mut self,
        f: impl FnOnce(Vec2) -> Vec2,
    ) {
        match self {
            MouseEvent::Down(_, x, y, ..)
            | MouseEvent::Up(_, x, y, ..)
            | MouseEvent::Drag(_, x, y, ..)
            | MouseEvent::ScrollUp(x, y, ..)
            | MouseEvent::ScrollDown(x, y, ..) => {
                let pos = Vec2::new(*x, *y);
                let pos = f(pos);
                *x = pos.x;
                *y = pos.y;
            }
        }
    }

    fn filter_map_pos(
        &mut self,
        f: impl FnOnce(Vec2) -> Option<Vec2>,
    ) -> bool {
        match self {
            MouseEvent::Down(_, x, y, ..)
            | MouseEvent::Up(_, x, y, ..)
            | MouseEvent::Drag(_, x, y, ..)
            | MouseEvent::ScrollUp(x, y, ..)
            | MouseEvent::ScrollDown(x, y, ..) => {
                let pos = Vec2::new(*x, *y);
                let pos = match f(pos) {
                    Some(pos) => pos,
                    None => return false,
                };
                *x = pos.x;
                *y = pos.y;
                true
            }
        }
    }

    fn try_left_down(&self) -> Option<Vec2> {
        match self {
            MouseEvent::Down(MouseButton::Left, x, y, ..) => Some((*x, *y).into()),
            _ => None,
        }
    }

    fn try_left_up(&self) -> Option<Vec2> {
        match self {
            MouseEvent::Up(MouseButton::Left, x, y, ..) => Some((*x, *y).into()),
            _ => None,
        }
    }

    fn from_left_down(pos: Vec2) -> Self {
        MouseEvent::Down(MouseButton::Left, pos.x, pos.y, KeyModifiers::empty())
    }

    fn from_left_up(pos: Vec2) -> Self {
        MouseEvent::Up(MouseButton::Left, pos.x, pos.y, KeyModifiers::empty())
    }

    fn try_drag(&self) -> Option<Vec2> {
        match self {
            MouseEvent::Drag(MouseButton::Left, x, y, ..) => Some((*x, *y).into()),
            _ => None,
        }
    }
}

impl EventLike for Event {
    type KeyEvent = KeyEvent;
    type MouseEvent = MouseEvent;

    fn try_mouse(&self) -> Option<&Self::MouseEvent> {
        match self {
            Event::Mouse(me) => Some(me),
            _ => None,
        }
    }

    fn try_mouse_mut(&mut self) -> Option<&mut Self::MouseEvent> {
        match self {
            Event::Mouse(me) => Some(me),
            _ => None,
        }
    }

    fn try_key(&self) -> Option<&Self::KeyEvent> {
        match self {
            Event::Key(ke) => Some(ke),
            _ => None,
        }
    }

    fn try_key_mut(&mut self) -> Option<&mut Self::KeyEvent> {
        match self {
            Event::Key(ke) => Some(ke),
            _ => None,
        }
    }

    fn try_resize(&self) -> Option<Vec2> {
        match self {
            Event::Resize(x, y) => Some(Vec2::new(*x, *y)),
            _ => None,
        }
    }
}
