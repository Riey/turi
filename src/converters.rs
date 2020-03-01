use crate::{
    event::Event,
    view::View,
};
use std::marker::PhantomData;

crate::impl_deref_for_inner!(Map<H, U, F>);
crate::impl_deref_for_inner!(OrElse<H, F>);
crate::impl_deref_for_inner!(OrElseFirst<H, F>);

pub struct Map<H, U, F> {
    inner:   H,
    f:       F,
    _marker: PhantomData<U>,
}

impl<H, U, F> Map<H, U, F> {
    pub fn new(
        inner: H,
        f: F,
    ) -> Self {
        Self {
            inner,
            f,
            _marker: PhantomData,
        }
    }
}

impl<S, H, U, F> View<S> for Map<H, U, F>
where
    H: View<S>,
    F: FnMut(&mut H, &mut S, H::Message) -> U,
{
    type Message = U;

    crate::impl_view_with_inner!(inner);

    #[inline]
    fn on_event(
        &mut self,
        state: &mut S,
        e: Event,
    ) -> Option<U> {
        let msg = self.inner.on_event(state, e);

        msg.map(|msg| (self.f)(&mut self.inner, state, msg))
    }
}

pub struct OrElseFirst<H, F> {
    inner: H,
    f:     F,
}

impl<H, F> OrElseFirst<H, F> {
    pub fn new(
        inner: H,
        f: F,
    ) -> Self {
        Self { inner, f }
    }
}

impl<S, H, F> View<S> for OrElseFirst<H, F>
where
    H: View<S>,
    F: FnMut(&mut H, &mut S, Event) -> Option<H::Message>,
{
    type Message = H::Message;

    crate::impl_view_with_inner!(inner);

    #[inline]
    fn on_event(
        &mut self,
        state: &mut S,
        e: Event,
    ) -> Option<Self::Message> {
        (self.f)(&mut self.inner, state, e).or_else(|| self.inner.on_event(state, e))
    }
}

pub struct OrElse<H, F> {
    inner: H,
    f:     F,
}

impl<H, F> OrElse<H, F> {
    pub fn new(
        inner: H,
        f: F,
    ) -> Self {
        Self { inner, f }
    }
}

impl<S, H, F> View<S> for OrElse<H, F>
where
    H: View<S>,
    F: FnMut(&mut H, &mut S, Event) -> Option<H::Message>,
{
    type Message = H::Message;

    crate::impl_view_with_inner!(inner);

    #[inline]
    fn on_event(
        &mut self,
        state: &mut S,
        e: Event,
    ) -> Option<Self::Message> {
        self.inner
            .on_event(state, e)
            .or_else(|| (self.f)(&mut self.inner, state, e))
    }
}
