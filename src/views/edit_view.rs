use crate::{
    event::EventHandler,
    events::{
        BackspaceEvent,
        CharEvent,
        EnterEvent,
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

impl<S> EventHandler<S, CharEvent> for EditView {
    type Message = EditViewMessage;

    fn on_event(
        &mut self,
        _: &mut S,
        event: CharEvent,
    ) -> Self::Message {
        self.text_mut().push(event.0);

        EditViewMessage::Edit
    }
}

impl<S> EventHandler<S, BackspaceEvent> for EditView {
    type Message = EditViewMessage;

    fn on_event(
        &mut self,
        _: &mut S,
        _: BackspaceEvent,
    ) -> Self::Message {
        self.text_mut().pop();

        EditViewMessage::Edit
    }
}

impl<S> EventHandler<S, EnterEvent> for EditView {
    type Message = EditViewMessage;

    fn on_event(
        &mut self,
        _: &mut S,
        _: EnterEvent,
    ) -> Self::Message {
        EditViewMessage::Submit
    }
}
