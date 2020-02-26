use crate::{
    never::Never,
    printer::Printer,
    vec2::Vec2,
    view::View,
};
use unicode_width::UnicodeWidthStr;

use ansi_term::ANSIString;
use std::marker::PhantomData;

pub struct TextView<'a, S, E> {
    text:       ANSIString<'a>,
    text_width: u16,
    _marker:    PhantomData<(S, E)>,
}

impl<'a, S, E> TextView<'a, S, E> {
    pub fn new(text: ANSIString<'a>) -> Self {
        let text_width = text.width() as u16;
        Self {
            text,
            text_width,
            _marker: PhantomData,
        }
    }
}

impl<'a, S, E> View<S, E> for TextView<'a, S, E> {
    type Message = Never;

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
        _event: E,
    ) -> Option<Never> {
        None
    }
}
