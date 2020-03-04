use crate::{
    printer::Printer,
    style::Style,
    vec2::Vec2,
    view::{
        EventResult,
        View,
        IGNORE,
    },
};
use unicode_width::UnicodeWidthStr;

use std::marker::PhantomData;

pub struct TextView<S, E> {
    text:       String,
    text_width: u16,
    _marker:    PhantomData<(S, E)>,
}

impl<S, E> TextView<S, E> {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let text_width = text.width() as u16;
        Self {
            text,
            text_width,
            _marker: PhantomData,
        }
    }
}

impl<S, E> View<S, E> for TextView<S, E> {
    #[inline]
    fn desired_size(&self) -> Vec2 {
        Vec2::new(self.text_width, 1)
    }

    #[inline]
    fn layout(
        &mut self,
        _size: Vec2,
    ) {
    }

    #[inline]
    fn render(
        &self,
        printer: &mut Printer,
    ) {
        printer.with_style(Style::view(), |printer| {
            printer.print((0, 0), &self.text);
        });
    }

    #[inline]
    fn on_event(
        &mut self,
        _state: &mut S,
        _event: E,
    ) -> EventResult {
        IGNORE
    }
}
