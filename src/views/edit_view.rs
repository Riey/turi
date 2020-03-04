use crate::{
    event::{
        EventLike,
        KeyEventLike,
    },
    event_result::{
        EventResult,
        REDRAW,
        NODRAW,
        IGNORE,
    },
    printer::Printer,
    style::Style,
    vec2::Vec2,
    view::{
        View,
    },
};
use std::marker::PhantomData;
use unicode_width::UnicodeWidthChar;

pub struct EditView<S, E, FE, FS> {
    text:    String,
    width:   usize,
    on_edit: FE,
    on_submut: FS,
    _marker: PhantomData<(S, E)>,
}

impl<S, E, FE, FS> EditView<S, E, FE, FS> {
    pub fn with_callback(on_edit: FE, on_submit: FS) -> Self {
        Self {
            text:    String::new(),
            width:   0,
            on_edit,
            on_submit,
            _marker: PhantomData,
        }
    }

    pub fn new() -> EditView<S, E, fn(&mut S, &str), fn(&mut S, &str)> {
        EditView::with_callback(|_, _| (), |_, _| ())
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl<S, E, FE, FS> View<S, E> for EditView<S, E, FE, FS>
where E: EventLike, FE: Fn(&mut S, &str), FS: Fn(&mut S, &str) {
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
        e: E,
    ) -> EventResult {
        let ke = e.try_key()?;
        if ke.try_enter() {
            Some(EditViewMessage::Submit)
        } else if let Some(ch) = ke.try_char() {
            self.text.push(ch);
            self.width += ch.width().unwrap_or(0);
            state.set_need_redraw(true);
            Some(EditViewMessage::Edit)
        } else if ke.try_backspace() {
            if let Some(ch) = self.text.pop() {
                self.width -= ch.width().unwrap_or(0);
                state.set_need_redraw(true);
                Some(EditViewMessage::Edit)
            } else {
                None
            }
        } else {
            None
        }
    }
}
