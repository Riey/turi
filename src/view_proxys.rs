use crate::{
    printer::Printer,
    vec2::Vec2,
    view::View,
};
use std::{
    marker::PhantomData,
    ops::Try,
};

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

    #[inline(always)]
    fn render(
        &self,
        printer: &mut Printer,
    ) {
        self.inner.render(printer);
    }

    #[inline(always)]
    fn layout(
        &mut self,
        size: Vec2,
    ) {
        self.inner.layout(size);
    }

    #[inline(always)]
    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size()
    }

    #[inline(always)]
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
    inner:   V,
    f:       F,
    _marker: PhantomData<E>,
}

impl<V, E, F> MapE<V, E, F> {
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

impl<S, V, E, F> View<S> for MapE<V, E, F>
where
    V: View<S>,
    F: FnMut(&mut V, &mut S, E) -> V::Event,
{
    type Event = E;
    type Message = V::Message;

    #[inline(always)]
    fn render(
        &self,
        printer: &mut Printer,
    ) {
        self.inner.render(printer);
    }

    #[inline(always)]
    fn layout(
        &mut self,
        size: Vec2,
    ) {
        self.inner.layout(size);
    }

    #[inline(always)]
    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size()
    }

    #[inline(always)]
    fn on_event(
        &mut self,
        state: &mut S,
        e: E,
    ) -> Self::Message {
        let e = (self.f)(&mut self.inner, state, e);
        self.inner.on_event(state, e)
    }
}

pub struct MapOptE<V, E, F> {
    inner:   V,
    f:       F,
    _marker: PhantomData<E>,
}

impl<V, E, F> MapOptE<V, E, F> {
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

impl<S, V, E, F> View<S> for MapOptE<V, E, F>
where
    V: View<S>,
    F: FnMut(&mut V, &mut S, E) -> Option<V::Event>,
{
    type Event = E;
    type Message = Option<V::Message>;

    #[inline(always)]
    fn render(
        &self,
        printer: &mut Printer,
    ) {
        self.inner.render(printer);
    }

    #[inline(always)]
    fn layout(
        &mut self,
        size: Vec2,
    ) {
        self.inner.layout(size);
    }

    #[inline(always)]
    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size()
    }

    #[inline(always)]
    fn on_event(
        &mut self,
        state: &mut S,
        e: E,
    ) -> Self::Message {
        let e = (self.f)(&mut self.inner, state, e);

        match e {
            Some(e) => Some(self.inner.on_event(state, e)),
            None => None,
        }
    }
}

pub struct OrElseFirst<V, F> {
    inner: V,
    f:     F,
}

impl<V, F> OrElseFirst<V, F> {
    pub fn new(
        inner: V,
        f: F,
    ) -> Self {
        Self { inner, f }
    }
}

impl<S, V, F, T: Try> View<S> for OrElseFirst<V, F>
where
    V: View<S, Message = T>,
    V::Event: Clone,
    F: FnMut(&mut V, &mut S, V::Event) -> T,
{
    type Event = V::Event;
    type Message = T;

    #[inline(always)]
    fn render(
        &self,
        printer: &mut Printer,
    ) {
        self.inner.render(printer);
    }

    #[inline(always)]
    fn layout(
        &mut self,
        size: Vec2,
    ) {
        self.inner.layout(size);
    }

    #[inline(always)]
    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size()
    }

    #[inline(always)]
    fn on_event(
        &mut self,
        state: &mut S,
        e: Self::Event,
    ) -> Self::Message {
        match (self.f)(&mut self.inner, state, e.clone()).into_result() {
            Ok(ret) => T::from_ok(ret),
            Err(_) => T::from_ok(self.inner.on_event(state, e)?),
        }
    }
}

pub struct OrElse<V, F> {
    inner: V,
    f:     F,
}

impl<V, F> OrElse<V, F> {
    pub fn new(
        inner: V,
        f: F,
    ) -> Self {
        Self { inner, f }
    }
}

impl<S, V, F, T: Try> View<S> for OrElse<V, F>
where
    V: View<S, Message = T>,
    V::Event: Clone,
    F: FnMut(&mut V, &mut S, V::Event) -> T,
{
    type Event = V::Event;
    type Message = T;

    #[inline(always)]
    fn render(
        &self,
        printer: &mut Printer,
    ) {
        self.inner.render(printer);
    }

    #[inline(always)]
    fn layout(
        &mut self,
        size: Vec2,
    ) {
        self.inner.layout(size);
    }

    #[inline(always)]
    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size()
    }

    #[inline(always)]
    fn on_event(
        &mut self,
        state: &mut S,
        e: Self::Event,
    ) -> Self::Message {
        match self.inner.on_event(state, e.clone()).into_result() {
            Ok(ret) => T::from_ok(ret),
            Err(_) => T::from_ok((self.f)(&mut self.inner, state, e)?),
        }
    }
}
