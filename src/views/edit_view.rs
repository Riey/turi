use crate::{
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditViewEvent {
    Char(char),
    Backspace,
    Enter,
}

impl<S> View<S> for EditView {
    type Event = EditViewEvent;
    type Message = EditViewMessage;

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

    fn on_event(
        &mut self,
        _state: &mut S,
        e: EditViewEvent,
    ) -> EditViewMessage {
        match e {
            EditViewEvent::Enter => EditViewMessage::Submit,
            EditViewEvent::Backspace => {
                self.text.pop();
                EditViewMessage::Edit
            }
            EditViewEvent::Char(ch) => {
                self.text.push(ch);
                EditViewMessage::Edit
            }
        }
    }
}
