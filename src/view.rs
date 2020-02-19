use crate::{
    printer::Printer,
    vec2::Vec2,
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
}

impl<'a, S, E, M> View<S> for Box<dyn View<S, Event = E, Message = M> + 'a> {
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
