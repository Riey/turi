use crate::{
    converters::{
        Map,
        MapE,
        MapOptE,
        OrElse,
        OrElseFirst,
    },
    orientation::Orientation,
    printer::Printer,
    vec2::Vec2,
    view_wrappers::{
        ConsumeEvent,
        ScrollView,
    },
};

pub trait View<S, E> {
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
        event: E,
    ) -> Option<Self::Message>;

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
    fn map_e<NE, F>(
        self,
        f: F,
    ) -> MapE<Self, NE, F>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, NE) -> E,
    {
        MapE::new(self, f)
    }

    #[inline]
    fn map_opt_e<NE, F>(
        self,
        f: F,
    ) -> MapOptE<Self, NE, F>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, NE) -> Option<E>,
    {
        MapOptE::new(self, f)
    }

    #[inline]
    fn or_else<F>(
        self,
        f: F,
    ) -> OrElse<Self, F>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, E) -> Option<Self::Message>,
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
        F: FnMut(&mut Self, &mut S, E) -> Option<Self::Message>,
    {
        OrElseFirst::new(self, f)
    }
}

pub trait ScrollableView<S, E>: View<S, E> {
    fn scroll_vertical_render(
        &self,
        pos: u16,
        printer: &mut Printer,
    );
    fn scroll_horizontal_render(
        &self,
        pos: u16,
        printer: &mut Printer,
    );
    fn scroll_both_render(
        &self,
        pos: Vec2,
        printer: &mut Printer,
    );

    #[inline(always)]
    fn scrollbar(
        self,
        orientation: Orientation,
    ) -> ScrollView<Self>
    where
        Self: Sized,
    {
        ScrollView::new(self, orientation)
    }
}

impl<S, E, M> View<S, E> for Box<dyn View<S, E, Message = M>> {
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
        event: E,
    ) -> Option<M> {
        (**self).on_event(state, event)
    }
}
