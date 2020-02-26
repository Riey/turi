use crate::{
    event::EventLike,
    printer::Printer,
    vec2::Vec2,
    view::View,
    state::RedrawState,
};
use ansi_term::{
    Color,
    Style,
};
use std::marker::PhantomData;
use unicode_width::UnicodeWidthStr;

pub struct SelectView<S, E, T> {
    btns:     Vec<(String, T)>,
    selected: usize,
    width:    u16,
    _marker:  PhantomData<(S, E)>,
}

impl<S: RedrawState, E, T> SelectView<S, E, T> {
    pub fn new() -> Self {
        Self {
            btns:     Vec::new(),
            selected: 0,
            width:    0,
            _marker:  PhantomData,
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
            _marker: PhantomData,
        }
    }

    pub fn focus_down(&mut self, state: &mut S) -> Option<SelectViewMessage> {
        let val = self.selected + 1;

        if val >= self.btns.len() {
            None
        } else {
            self.selected = val;
            state.set_need_redraw(true);
            Some(SelectViewMessage::IndexChanged)
        }
    }

    pub fn focus_up(&mut self, state: &mut S) -> Option<SelectViewMessage> {
        if self.selected > 0 {
            self.selected -= 1;
            state.set_need_redraw(true);
            Some(SelectViewMessage::IndexChanged)
        } else {
            None
        }
    }

    pub fn selected_val(&self) -> &T {
        &self.btns[self.selected].1
    }

    pub fn selected_val_mut(&mut self) -> &mut T {
        &mut self.btns[self.selected].1
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectViewMessage {
    Select,
    IndexChanged,
}

impl<S: RedrawState, E: EventLike, T> View<S, E> for SelectView<S, E, T> {
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
        state: &mut S,
        e: E,
    ) -> Option<Self::Message> {
        if e.try_mouse_down().is_some() || e.try_enter() {
            Some(SelectViewMessage::Select)
        } else if e.try_up() {
            self.focus_up(state)
        } else if e.try_down() {
            self.focus_down(state)
        } else {
            None
        }
    }
}
