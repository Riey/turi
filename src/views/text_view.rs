use crate::{
    printer::Printer,
    vec2::Vec2,
    view::View,
};
use unicode_width::UnicodeWidthStr;

use ansi_term::ANSIString;

pub struct TextView<'a> {
    text:       ANSIString<'a>,
    text_width: u16,
}

impl<'a> TextView<'a> {
    pub fn new(text: ANSIString<'a>) -> Self {
        let text_width = text.width() as u16;
        Self { text, text_width }
    }
}

impl<'a, S> View<S> for TextView<'a> {
    type Event = ();
    type Message = ();

    fn desired_size(&self) -> Vec2 {
        Vec2::new(self.text_width, 1)
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
        printer.print_styled((0, 0), &self.text);
    }

    fn on_event(
        &mut self,
        _state: &mut S,
        _event: (),
    ) {
    }
}
