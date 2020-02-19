use crate::{
    printer::Printer,
    vec2::Vec2,
    view::View,
};
use ansi_term::{
    Color,
    Style,
};
use unicode_width::UnicodeWidthStr;

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

    pub fn focus_down(&mut self) -> SelectViewMessage {
        let val = self.selected + 1;

        if val >= self.btns.len() {
            SelectViewMessage::Nop
        } else {
            self.selected = val;
            SelectViewMessage::IndexChanged
        }
    }

    pub fn focus_up(&mut self) -> SelectViewMessage {
        if self.selected > 0 {
            self.selected -= 1;
            SelectViewMessage::IndexChanged
        } else {
            SelectViewMessage::Nop
        }
    }

    pub fn selected_val(&self) -> &T {
        &self.btns[self.selected].1
    }

    pub fn selected_val_mut(&mut self) -> &mut T {
        &mut self.btns[self.selected].1
    }
}

pub enum SelectViewEvent {
    Up,
    Down,
    Enter,
    Click(u16),
}

pub enum SelectViewMessage {
    Select,
    IndexChanged,
    Nop,
}

impl<S, T> View<S> for SelectView<T> {
    type Event = SelectViewEvent;
    type Message = SelectViewMessage;

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        for (i, (text, _)) in self.btns.iter().enumerate() {
            if i == self.selected {
                printer.with_style(Style::new().on(Color::Yellow), |printer| {
                    printer.print((0, i as u16), text);
                })
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
        e: Self::Event,
    ) -> Self::Message {
        match e {
            SelectViewEvent::Enter => SelectViewMessage::Select,
            SelectViewEvent::Click(idx) => {
                self.selected = idx as usize;
                SelectViewMessage::Select
            }
            SelectViewEvent::Up => self.focus_up(),
            SelectViewEvent::Down => self.focus_down(),
        }
    }
}
