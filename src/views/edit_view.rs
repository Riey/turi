use crate::{
    printer::Printer,
    style::Style,
    vec2::Vec2,
    view::View,
};
use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
};
use unicode_width::UnicodeWidthStr;

pub struct EditView {
    text:  String,
    style: Style,
}

impl EditView {
    pub fn new() -> Self {
        Self {
            text:  String::new(),
            style: Style::default(),
        }
    }

    pub fn with_style(
        mut self,
        style: Style,
    ) -> Self {
        self.style = style;
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditViewEvent {
    Edit,
    Submit,
}

impl<S> View<S> for EditView {
    type Message = EditViewEvent;

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
        printer.with_style(self.style, |printer| {
            printer.print((0, 0), &self.text);
        });
    }

    fn on_event(
        &mut self,
        _state: &mut S,
        e: Event,
    ) -> Option<Self::Message> {
        match e {
            // TODO: mouse
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => Some(EditViewEvent::Submit),
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => {
                self.text.pop();
                Some(EditViewEvent::Edit)
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                modifiers,
            }) if modifiers.is_empty() => {
                self.text.push(ch);
                Some(EditViewEvent::Edit)
            }
            _ => None,
        }
    }
}
