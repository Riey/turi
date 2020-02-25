use crate::{
    event::{
        EventHandler,
        EventLike,
    },
    printer::Printer,
    vec2::Vec2,
    view::View, state::RedrawState,
};
use unicode_width::UnicodeWidthChar;

pub struct EditView {
    text:  String,
    width: usize,
}

impl EditView {
    pub fn new() -> Self {
        Self {
            text:  String::new(),
            width: 0,
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

impl View for EditView {
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
}

impl<S: RedrawState, E: EventLike> EventHandler<S, E> for EditView {
    type Message = EditViewMessage;

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
