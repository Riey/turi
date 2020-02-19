use crate::{
    printer::Printer,
    vec2::Vec2,
    view::View,
};
use unicode_width::UnicodeWidthStr;

pub struct ButtonView {
    text:       String,
    text_width: u16,
}

impl ButtonView {
    pub fn new(
        mut text: String,
        decoration: ButtonDecoration,
    ) -> Self {
        decoration.decoration(&mut text);

        let text_width = text.width() as u16;

        Self { text, text_width }
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

pub enum ButtonViewEvent {
    Click,
}

impl<S> View<S> for ButtonView {
    type Event = ButtonViewEvent;
    type Message = ButtonViewEvent;

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
        printer.print((0, 0), &self.text);
    }

    fn on_event(
        &mut self,
        _state: &mut S,
        e: Self::Event,
    ) -> Self::Message {
        e
    }
}
