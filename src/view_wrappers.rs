use crate::{
    printer::Printer,
    rect::Rect,
    vec2::Vec2,
    view::View,
};
use std::cell::Cell;

impl_deref_for_generic_inner!(SizeCacher => inner);
impl_deref_for_generic_inner!(BoundChecker => inner);

pub struct SizeCacher<T> {
    inner:     T,
    prev_size: Vec2,
}

impl<T> SizeCacher<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            prev_size: Vec2::new(0, 0),
        }
    }

    #[inline]
    pub fn prev_size(&self) -> Vec2 {
        self.prev_size
    }
}

impl<S, E, T> View<S> for SizeCacher<T>
where
    T: View<S, Event = E>,
{
    type Event = E;
    type Message = T::Message;

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        self.inner.render(printer);
    }

    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size()
    }

    fn layout(
        &mut self,
        size: Vec2,
    ) {
        self.prev_size = size;
        self.inner.layout(size);
    }

    fn on_event(
        &mut self,
        state: &mut S,
        e: E,
    ) -> T::Message {
        self.inner.on_event(state, e)
    }
}

pub struct BoundChecker<T> {
    inner: T,
    bound: Cell<Rect>,
}

impl<T> BoundChecker<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            bound: Cell::new(Rect::new((0, 0), (0, 0))),
        }
    }

    pub fn contains(
        &self,
        p: Vec2,
    ) -> bool {
        self.bound.get().contains(p)
    }
}

impl<S, T> View<S> for BoundChecker<T>
where
    T: View<S, Event = bool>,
{
    type Event = Vec2;
    type Message = T::Message;

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        self.bound.set(printer.bound());
        self.inner.render(printer);
    }

    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size()
    }

    fn layout(
        &mut self,
        size: Vec2,
    ) {
        self.inner.layout(size);
    }

    fn on_event(
        &mut self,
        state: &mut S,
        e: Vec2,
    ) -> T::Message {
        self.inner.on_event(state, self.contains(e))
    }
}
