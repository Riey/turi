use crate::{
    event::{
        Event,
        KeyCode,
        KeyEvent,
        KeyEventType,
    },
    printer::Printer,
    state::RedrawState,
    style::Style,
    vec2::Vec2,
    view::View,
};
use std::marker::PhantomData;
use unicode_width::UnicodeWidthChar;

pub struct EditView<S> {
    text:    String,
    width:   usize,
    _marker: PhantomData<S>,
}

impl<S> EditView<S> {
    pub fn new() -> Self {
        Self {
            text:    String::new(),
            width:   0,
            _marker: PhantomData,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditViewMessage {
    Edit,
    Submit,
}

impl<S: RedrawState> View<S> for EditView<S> {
    type Message = EditViewMessage;

    fn desired_size(&self) -> Vec2 {
        Vec2::new(self.width as u16, 1)
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
        printer.with_style(Style::view(), |printer| {
            printer.print((0, 0), &self.text);
        });
    }

    fn on_event(
        &mut self,
        state: &mut S,
        e: Event,
    ) -> Option<Self::Message> {
        match e {
            Event::Mouse(..) => todo!(),
            Event::Key(KeyEvent(ty, modi)) if modi.is_empty() => {
                match ty {
                    KeyEventType::Key(KeyCode::Enter) => Some(EditViewMessage::Submit),
                    KeyEventType::Key(KeyCode::Backspace) => {
                        if let Some(ch) = self.text.pop() {
                            self.width -= ch.width().unwrap_or(0);
                            state.set_need_redraw(true);
                            Some(EditViewMessage::Edit)
                        } else {
                            None
                        }
                    }
                    KeyEventType::Char(ch) => {
                        self.text.push(ch);
                        self.width += ch.width().unwrap_or(0);
                        state.set_need_redraw(true);
                        Some(EditViewMessage::Edit)
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
