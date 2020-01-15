use crate::view::{View, ViewProxy};
use crossterm::event::Event;
use std::marker::PhantomData;

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

impl<'a, V, F, U> ViewProxy for Map<V, F, U>
where
    V: View,
    F: FnMut(&mut V, V::Message) -> U + 'a,
{
    type Inner = V;
    type Message = U;

    fn inner_view(&self) -> &Self::Inner {
        &self.inner
    }

    fn inner_view_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }

    fn proxy_on_event(&mut self, e: Event) -> Option<U> {
        let msg = self.inner.on_event(e);
        msg.map(|msg| (self.f)(&mut self.inner, msg))
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

impl<'a, V, F> ViewProxy for MapE<V, F>
where
    V: View,
    F: FnMut(Event) -> Option<V::Message> + 'a,
{
    type Inner = V;
    type Message = V::Message;

    fn inner_view(&self) -> &Self::Inner {
        &self.inner
    }

    fn inner_view_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }

    fn proxy_on_event(&mut self, e: Event) -> Option<V::Message> {
        (self.f)(e).or_else(|| self.inner.on_event(e))
    }
}
