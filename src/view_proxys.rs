use crate::{
    printer::Printer,
    vec2::Vec2,
    view::View,
};
use std::marker::PhantomData;

pub struct Map<V, U, F> {
    inner:   V,
    f:       F,
    _marker: PhantomData<U>,
}

impl<V, U, F> Map<V, U, F> {
    pub fn new(
        inner: V,
        f: F,
    ) -> Self {
        Self {
            inner,
            f,
            _marker: PhantomData,
        }
    }
}

impl<S, V, U, F> View<S> for Map<V, U, F>
where
    V: View<S>,
    F: FnMut(&mut V, &mut S, V::Message) -> U,
{
    type Event = V::Event;
    type Message = U;

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        self.inner.render(printer);
    }

    fn layout(
        &mut self,
        size: Vec2,
    ) {
        self.inner.layout(size);
    }

    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size()
    }

    fn on_event(
        &mut self,
        state: &mut S,
        e: V::Event,
    ) -> U {
        let msg = self.inner.on_event(state, e);
        (self.f)(&mut self.inner, state, msg)
    }
}

pub struct MapE<V, E, F> {
    inner: V,
    f:     F,
    _marker: PhantomData<E>,
}

impl<V, E, F> MapE<V, E, F> {
    pub fn new(
        inner: V,
        f: F,
    ) -> Self {
        Self { inner, f, _marker: PhantomData }
    }
}

impl<S, V, E, F> View<S> for MapE<V, E, F>
where
    V: View<S>,
    F: FnMut(&mut V, &mut S, E) -> V::Event,
{
    type Event = E;
    type Message = V::Message;

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        self.inner.render(printer);
    }

    fn layout(
        &mut self,
        size: Vec2,
    ) {
        self.inner.layout(size);
    }

    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size()
    }

    fn on_event(
        &mut self,
        state: &mut S,
        e: E,
    ) -> V::Message {
        let e = (self.f)(&mut self.inner, state, e);
        self.inner.on_event(state, e)
    }
}
