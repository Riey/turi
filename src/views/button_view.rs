use crate::{
    event::{
        EventLike,
        KeyEventLike,
        MouseEventLike,
    },
    printer::Printer,
    style::Style,
    vec2::Vec2,
    view::View,
};
use std::marker::PhantomData;
use unicode_width::UnicodeWidthStr;

pub struct ButtonView<S, E> {
    text:       String,
    text_width: u16,
    _marker:    PhantomData<(S, E)>,
}

impl<S, E> ButtonView<S, E> {
    pub fn new(
        mut text: String,
        decoration: ButtonDecoration,
    ) -> Self {
        decoration.decoration(&mut text);

        let text_width = text.width() as u16;

        Self {
            text,
            text_width,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[inline]
    pub fn width(&self) -> u16 {
        self.text_width
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonDecoration {
    NoDecoration,
    Angle,
}

impl ButtonDecoration {
    #[inline]
    fn decoration(
        &self,
        text: &mut String,
    ) {
        match self {
            ButtonDecoration::NoDecoration => {}
            ButtonDecoration::Angle => {
                text.insert(0, '<');
                text.push('>');
            }
        }
    }
}

impl Default for ButtonDecoration {
    fn default() -> Self {
        Self::Angle
    }
}

impl<S, E: EventLike> View<S, E> for ButtonView<S, E> {
    type Message = ();

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
        e: E,
    ) -> Option<()> {
        if e.try_key().map(|ke| ke.try_enter()).unwrap_or(false) {
            Some(())
        } else {
            e.try_mouse()?.try_left_down().map(|_| ())
        }
    }
}
