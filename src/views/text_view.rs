use crate::{
    printer::Printer,
    style::StyledText,
    vec2::Vec2,
    view::View,
};

use crossterm::event::Event;

pub struct TextView {
    text: StyledText,
}

impl TextView {
    pub fn new(text: StyledText) -> Self {
        Self { text }
    }
}

impl<S> View<S> for TextView {
    type Message = ();

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
        printer.print_styled((0, 0), &self.text);
    }

    fn on_event(
        &mut self,
        _state: &mut S,
        _event: Event,
    ) -> Option<Self::Message> {
        None
    }
}
