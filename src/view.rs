use crate::{
    printer::Printer,
    vec2::Vec2,
    view_proxys::{
        Map,
        MapE,
        MapOptE,
        OrElse,
    },
};
use std::ops::Try;

pub trait View<S> {
    type Event;
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
        e: Self::Event,
    ) -> Self::Message;

    #[inline(always)]
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

    #[inline(always)]
    fn map_e<E, F>(
        self,
        f: F,
    ) -> MapE<Self, E, F>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, E) -> Self::Event,
    {
        MapE::new(self, f)
    }

    #[inline(always)]
    fn map_opt_e<E, F>(
        self,
        f: F,
    ) -> MapOptE<Self, E, F>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, E) -> Option<Self::Event>,
    {
        MapOptE::new(self, f)
    }

    #[inline(always)]
    fn or_else<F>(
        self,
        f: F,
    ) -> OrElse<Self, F>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, Self::Event) -> Self::Message,
        Self::Message: Try,
        Self::Event: Clone,
    {
        OrElse::new(self, f)
    }
}

impl<S, E, M> View<S> for Box<dyn View<S, Event = E, Message = M>> {
    type Event = E;
    type Message = M;

    #[inline(always)]
    fn render(
        &self,
        printer: &mut Printer,
    ) {
        (**self).render(printer)
    }

    #[inline(always)]
    fn layout(
        &mut self,
        size: Vec2,
    ) {
        (**self).layout(size)
    }

    #[inline(always)]
    fn desired_size(&self) -> Vec2 {
        (**self).desired_size()
    }

    #[inline(always)]
    fn on_event(
        &mut self,
        state: &mut S,
        e: Self::Event,
    ) -> Self::Message {
        (**self).on_event(state, e)
    }
}

impl<'a, S, V> View<S> for &'a mut V
where
    V: View<S>,
{
    type Event = V::Event;
    type Message = V::Message;

    #[inline(always)]
    fn render(
        &self,
        printer: &mut Printer,
    ) {
        (**self).render(printer)
    }

    #[inline(always)]
    fn layout(
        &mut self,
        size: Vec2,
    ) {
        (**self).layout(size)
    }

    #[inline(always)]
    fn desired_size(&self) -> Vec2 {
        (**self).desired_size()
    }

    #[inline(always)]
    fn on_event(
        &mut self,
        state: &mut S,
        e: Self::Event,
    ) -> Self::Message {
        (**self).on_event(state, e)
    }
}
