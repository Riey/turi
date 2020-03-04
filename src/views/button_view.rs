use crate::{
    event::{
        EventLike,
        KeyEventLike,
        MouseEventLike,
    },
    event_result::{
        EventResult,
        NODRAW,
    },
    printer::Printer,
    style::Style,
    vec2::Vec2,
    view::View,
};
use std::marker::PhantomData;
use unicode_width::UnicodeWidthStr;

pub struct ButtonView<S, E, F> {
    text:       String,
    text_width: u16,
    on_click:   F,
    _marker:    PhantomData<(S, E)>,
}

impl<S, E, F> ButtonView<S, E, F> {
    pub fn with_callback(
        text: impl Into<String>,
        on_click: F,
    ) -> Self {
        let text = text.into();
        let text_width = text.width() as u16;
        Self {
            text,
            text_width,
            on_click,
            _marker: PhantomData,
        }
    }

    pub fn new(text: impl Into<String>) -> ButtonView<S, E, fn(&mut S)> {
        ButtonView::with_on_click(text, |_| ())
    }

    pub fn on_click<NF>(
        self,
        f: NF,
    ) -> ButtonView<S, E, NF>
    where
        NF: Fn(&mut S),
    {
        ButtonView {
            text:       self.text,
            text_width: self.text_width,
            on_click:   f,
            _marker:    PhantomData,
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

impl<S, E: EventLike, F> View<S, E> for ButtonView<S, E, F>
where
    F: Fn(&mut S),
{
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
        state: &mut S,
        e: E,
    ) -> EventResult {
        if e.try_key().map(|ke| ke.try_enter()).unwrap_or(false)
            || e.try_mouse()
                .and_then(|m| m.try_left_down())
                .map_or(false, |_| true)
        {
            (self.on_click)(state);
        }

        NODRAW
    }
}
