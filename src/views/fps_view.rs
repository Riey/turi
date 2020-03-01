use crate::{
    event::Event,
    never::Never,
    style::Style,
    vec2::Vec2,
    view::View,
};
use std::{
    cell::Cell,
    marker::PhantomData,
    time::Instant,
};

pub struct FpsView<S> {
    prev_draw: Cell<Instant>,
    _marker:   PhantomData<S>,
}

impl<S> FpsView<S> {
    pub fn new() -> Self {
        Self {
            prev_draw: Cell::new(Instant::now()),
            _marker:   PhantomData,
        }
    }
}

impl<S> View<S> for FpsView<S> {
    type Message = Never;

    fn render(
        &self,
        printer: &mut crate::printer::Printer,
    ) {
        let now = Instant::now();
        let diff = now - self.prev_draw.get();
        let fps = 1.0f32 / diff.as_secs_f32();
        self.prev_draw.set(now);
        printer.with_style(Style::highlight(), |printer| {
            printer.print((0, 0), &format!("{:05.1}", fps));
        });
    }

    fn layout(
        &mut self,
        _size: crate::vec2::Vec2,
    ) {
    }

    fn desired_size(&self) -> Vec2 {
        Vec2::new(5, 1)
    }

    fn on_event(
        &mut self,
        _state: &mut S,
        _event: Event,
    ) -> Option<Self::Message> {
        None
    }
}
