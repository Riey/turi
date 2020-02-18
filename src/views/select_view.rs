use crate::{
    printer::Printer,
    style::Style,
    vec2::Vec2,
    view::View,
};
use crossterm::{
    event::{
        Event,
        KeyCode,
        KeyEvent,
    },
    style::Color,
};

use unicode_width::UnicodeWidthStr;

pub struct SelectEvent;

pub struct SelectView<T> {
    btns:     Vec<(String, T)>,
    selected: usize,
    width:    u16,
}

impl<T> SelectView<T> {
    pub fn new() -> Self {
        Self {
            btns:     Vec::new(),
            selected: 0,
            width:    0,
        }
    }

    pub fn with_items<I: IntoIterator<Item = (String, T)>>(items: I) -> Self {
        let btns: Vec<_> = items.into_iter().collect();

        let mut width = 0;

        for (text, _) in btns.iter() {
            width = width.max(text.width() as u16);
        }

        Self {
            btns,
            selected: 0,
            width,
        }
    }

    pub fn focus_down(&mut self) {
        self.selected = (self.selected + 1).min(self.btns.len() - 1);
    }

    pub fn focus_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn selected_val(&self) -> &T {
        &self.btns[self.selected].1
    }

    pub fn selected_val_mut(&mut self) -> &mut T {
        &mut self.btns[self.selected].1
    }
}

impl<S, T> View<S> for SelectView<T> {
    type Message = SelectEvent;

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        for (i, (text, _)) in self.btns.iter().enumerate() {
            if i == self.selected {
                printer.with_style(
                    Style {
                        bg: Color::DarkYellow,
                        ..Default::default()
                    },
                    |printer| {
                        printer.print((0, i as u16), text);
                    },
                )
            } else {
                printer.print((0, i as u16), text);
            }
        }
    }

    fn layout(
        &mut self,
        _size: Vec2,
    ) {
    }

    fn desired_size(&self) -> Vec2 {
        Vec2::new(self.width, self.btns.len() as u16)
    }

    fn on_event(
        &mut self,
        _state: &mut S,
        e: Event,
    ) -> Option<SelectEvent> {
        match e {
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => Some(SelectEvent),
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                self.focus_up();
                None
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                self.focus_down();
                None
            }
            _ => None,
        }
    }
}
