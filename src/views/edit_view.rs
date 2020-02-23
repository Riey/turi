use crate::{
    event::{
        EventHandler,
        EventLike,
    },
    printer::Printer,
    vec2::Vec2,
    view::View,
};
use unicode_width::UnicodeWidthStr;

pub struct EditView {
    text: String,
}

impl EditView {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditViewMessage {
    Edit,
    Submit,
}

impl View for EditView {
    fn desired_size(&self) -> Vec2 {
        Vec2::new(self.text.width() as u16, 1)
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

impl<S, E: EventLike> EventHandler<S, E> for EditView {
    type Message = EditViewMessage;

    fn on_event(
        &mut self,
        _: &mut S,
        e: E,
    ) -> Option<Self::Message> {
        if e.try_enter() {
            Some(EditViewMessage::Submit)
        } else if let Some(ch) = e.try_char() {
            self.text_mut().push(ch);
            Some(EditViewMessage::Edit)
        } else if e.try_backspace() {
            if self.text.pop().is_some() {
                Some(EditViewMessage::Edit)
            } else {
                None
            }
        } else {
            None
        }
    }
}
