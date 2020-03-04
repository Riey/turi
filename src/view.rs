use crate::{
    event_result::EventResult,
    orientation::Orientation,
    printer::Printer,
    vec2::Vec2,
    view_wrappers::ScrollView,
};

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
