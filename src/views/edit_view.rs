use crate::{
    event::EventLike,
    printer::Printer,
    state::RedrawState,
    vec2::Vec2,
    view::View,
};
use std::marker::PhantomData;
use unicode_width::UnicodeWidthChar;

pub struct EditView<S, E> {
    text:    String,
    width:   usize,
    _marker: PhantomData<(S, E)>,
}

impl<S, E> EditView<S, E> {
    pub fn new() -> Self {
        Self {
            text:    String::new(),
            width:   0,
            _marker: PhantomData,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditViewMessage {
    Edit,
    Submit,
}

impl<S: RedrawState, E: EventLike> View<S, E> for EditView<S, E> {
    type Message = EditViewMessage;

    fn desired_size(&self) -> Vec2 {
        Vec2::new(self.width as u16, 1)
    }

    fn layout(
        &mut self,
        _size: Vec2,
    ) {
    }

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        printer.print((0, 0), &self.text);
    }

    fn on_event(
        &mut self,
        state: &mut S,
        e: E,
    ) -> Option<Self::Message> {
        if e.try_enter() {
            Some(EditViewMessage::Submit)
        } else if let Some(ch) = e.try_char() {
            self.text.push(ch);
            self.width += ch.width().unwrap_or(0);
            state.set_need_redraw(true);
            Some(EditViewMessage::Edit)
        } else if e.try_backspace() {
            if let Some(ch) = self.text.pop() {
                self.width -= ch.width().unwrap_or(0);
                state.set_need_redraw(true);
                Some(EditViewMessage::Edit)
            } else {
                None
            }
        } else {
            None
        }
    }
}
