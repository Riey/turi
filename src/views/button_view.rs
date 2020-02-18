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

pub struct ButtonView {
    text:       String,
    text_width: u16,
    style:      Style,
}

impl ButtonView {
    pub fn new(
        mut text: String,
        decoration: ButtonDecoration,
    ) -> Self {
        decoration.decoration(&mut text);

        let text_width = text.width() as u16;

        Self {
            text,
            text_width,
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

pub enum ButtonEvent {
    Click,
}

impl<S> View<S> for ButtonView {
    type Message = ButtonEvent;

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
            })
            | Event::Mouse(..) => Some(ButtonEvent::Click),
            _ => None,
        }
    }
}
