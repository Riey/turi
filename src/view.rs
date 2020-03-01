use crate::{
    converters::{
        Map,
        OrElse,
        OrElseFirst,
    },
    event::Event,
    orientation::Orientation,
    printer::Printer,
    vec2::Vec2,
    view_wrappers::{
        ConsumeEvent,
        ScrollView,
    },
};

pub trait View<S> {
    type Message;

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
        event: Event,
    ) -> Option<Self::Message>;

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

    #[inline]
    fn consume_event<M>(
        self,
        msg: M,
    ) -> ConsumeEvent<Self, M>
    where
        Self: Sized,
        M: Clone,
    {
        ConsumeEvent::new(self, msg)
    }

    #[inline]
    fn map<U, F>(
        self,
        f: F,
    ) -> Map<Self, U, F>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, Self::Message) -> U,
    {
        Map::new(self, f)
    }

    #[inline]
    fn or_else<F>(
        self,
        f: F,
    ) -> OrElse<Self, F>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, Event) -> Option<Self::Message>,
    {
        OrElse::new(self, f)
    }

    #[inline]
    fn or_else_first<F>(
        self,
        f: F,
    ) -> OrElseFirst<Self, F>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, Event) -> Option<Self::Message>,
    {
        OrElseFirst::new(self, f)
    }
}

impl<S, M> View<S> for Box<dyn View<S, Message = M>> {
    type Message = M;

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
        event: Event,
    ) -> Option<M> {
        (**self).on_event(state, event)
    }
}
