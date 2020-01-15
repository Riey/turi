use crate::view::{View, };
use crossterm::event::Event;
use std::marker::PhantomData;
use crate::vec2::Vec2;
use crate::printer::Printer;

pub struct Map<V, F, U> {
    inner: V,
    f: F,
    _marker: PhantomData<U>,
}

impl<V, F, U> Map<V, F, U> {
    pub fn new(inner: V, f: F) -> Self {
        Self {
            inner,
            f,
            _marker: PhantomData,
        }
    }
}

impl<'a, S, V, F, U> View<S> for Map<V, F, U>
where
    V: View<S>,
    F: FnMut(&mut V, &mut S, V::Message) -> U + 'a,
{
    type Message = U;

    fn render(&self, printer: &mut Printer) {
        self.inner.render(printer);
    }

    fn layout(&mut self, size: Vec2) {
        self.inner.layout(size);
    }

    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size()
    }

    fn on_event(&mut self, state: &mut S, e: Event) -> Option<U> {
        let msg = self.inner.on_event(state, e);
        msg.map(|msg| (self.f)(&mut self.inner, state, msg))
    }
}

pub struct MapE<V, F> {
    inner: V,
    f: F,
}

impl<V, F> MapE<V, F> {
    pub fn new(inner: V, f: F) -> Self {
        Self { inner, f }
    }
}

impl<'a, S, V, F> View<S> for MapE<V, F>
where
    V: View<S>,
    F: FnMut(&mut S, Event) -> Option<V::Message> + 'a,
{
    type Message = V::Message;

    fn render(&self, printer: &mut Printer) {
        self.inner.render(printer);
    }

    fn layout(&mut self, size: Vec2) {
        self.inner.layout(size);
    }

    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size()
    }

    fn on_event(&mut self, state: &mut S, e: Event) -> Option<V::Message> {
        (self.f)(state, e).or_else(|| self.inner.on_event(state, e))
    }
}
