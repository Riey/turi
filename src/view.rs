use crate::{
    printer::Printer,
    vec2::Vec2,
    view_proxys::{Map, MapE},
};
use crossterm::event::Event;

pub trait View<S> {
    type Message;

    fn render(&self, printer: &mut Printer);
    fn layout(&mut self, size: Vec2);
    fn desired_size(&self) -> Vec2;
    fn on_event(&mut self, state: &mut S, e: Event) -> Option<Self::Message>;

    fn map<F, U>(self, f: F) -> Map<Self, F, U>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, Self::Message) -> U,
    {
        Map::new(self, f)
    }

    fn map_e<F>(self, f: F) -> MapE<Self, F>
    where
        Self: Sized,
        F: FnMut(&mut S, Event) -> Option<Self::Message>,
    {
        MapE::new(self, f)
    }
}

impl<'a, S, M> View<S> for Box<dyn View<S, Message = M> + 'a> {
    type Message = M;

    fn render(&self, printer: &mut Printer) {
        (**self).render(printer)
    }
    fn layout(&mut self, size: Vec2) {
        (**self).layout(size)
    }
    fn desired_size(&self) -> Vec2 {
        (**self).desired_size()
    }
    fn on_event(&mut self, state: &mut S, e: Event) -> Option<Self::Message> {
        (**self).on_event(state, e)
    }
}
