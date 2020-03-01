use crate::event::Event;
use crate::{
    never::Never,
    printer::Printer,
    style::Style,
    vec2::Vec2,
    view::View,
};
use unicode_width::UnicodeWidthStr;

use std::marker::PhantomData;

pub struct TextView<S> {
    text:       String,
    text_width: u16,
    _marker:    PhantomData<S>,
}

impl<S> TextView<S> {
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

impl<S> View<S> for TextView<S> {
    type Message = Never;

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
        _event: Event,
    ) -> Option<Never> {
        None
    }
}
