use crate::{
    printer::Printer,
    vec2::Vec2,
    view_proxys::{Map, MapE},
};

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

    fn map<U, F>(self, f: F) -> Map<Self, U, F> where Self: Sized, F: FnMut(&mut Self, &mut S, Self::Message) -> U {
        Map::new(self, f)
    }

    fn map_e<E, F>(self, f: F) -> MapE<Self, E, F> where Self: Sized, F: FnMut(&mut Self, &mut S, Self::Event) -> E {
        MapE::new(self, f)
    }
}

impl<S, E, M> View<S> for Box<dyn View<S, Event = E, Message = M>> {
    type Event = E;
    type Message = M;

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        (**self).render(printer)
    }

    fn layout(
        &mut self,
        size: Vec2,
    ) {
        (**self).layout(size)
    }

    fn desired_size(&self) -> Vec2 {
        (**self).desired_size()
    }

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

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        (**self).render(printer)
    }

    fn layout(
        &mut self,
        size: Vec2,
    ) {
        (**self).layout(size)
    }

    fn desired_size(&self) -> Vec2 {
        (**self).desired_size()
    }

    fn on_event(
        &mut self,
        state: &mut S,
        e: Self::Event,
    ) -> Self::Message {
        (**self).on_event(state, e)
    }
}
