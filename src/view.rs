use crate::{
    orientation::Orientation,
    printer::Printer,
    vec2::Vec2,
    view_wrappers::ScrollView,
};
pub const REDRAW: EventResult = EventResult::Consume(true);
pub const NODRAW: EventResult = EventResult::Consume(false);
pub const IGNORE: EventResult = EventResult::Ignored;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventResult {
    Consume(bool),
    Ignored,
}

impl EventResult {
    #[inline]
    pub fn is_consume(self) -> bool {
        !self.is_ignored()
    }

    #[inline]
    pub fn is_ignored(self) -> bool {
        self == IGNORE
    }

    #[inline]
    pub fn is_redraw(self) -> bool {
        self == REDRAW
    }

    #[inline]
    pub fn is_nodraw(self) -> bool {
        self == NODRAW
    }
}

pub trait View<S, E> {
    fn render(
        &self,
        printer: &mut Printer,
    );
    fn layout(
        &mut self,
        size: Vec2,
    );
    fn desired_size(&self) -> Vec2;

    fn on_event(
        &mut self,
        state: &mut S,
        event: E,
    ) -> EventResult;

    #[inline]
    fn scrollable(
        self,
        orientation: Orientation,
    ) -> ScrollView<Self>
    where
        Self: Sized,
    {
        ScrollView::new(self, orientation)
    }
}

impl<S, E> View<S, E> for Box<dyn View<S, E>> {
    #[inline]
    fn desired_size(&self) -> Vec2 {
        (**self).desired_size()
    }

    #[inline]
    fn layout(
        &mut self,
        size: Vec2,
    ) {
        (**self).layout(size)
    }

    #[inline]
    fn render(
        &self,
        printer: &mut Printer,
    ) {
        (**self).render(printer);
    }

    #[inline]
    fn on_event(
        &mut self,
        state: &mut S,
        event: E,
    ) -> EventResult {
        (**self).on_event(state, event)
    }
}
