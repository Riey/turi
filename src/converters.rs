use crate::view::View;
use std::marker::PhantomData;

crate::impl_deref_for_inner!(Map<H, U, F>);
crate::impl_deref_for_inner!(MapE<H, NE, F>);
crate::impl_deref_for_inner!(MapOptE<H, NE, F>);
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

impl<S, E, H, U, F> View<S, E> for Map<H, U, F>
where
    H: View<S, E>,
    F: FnMut(&mut H, &mut S, H::Message) -> U,
{
    type Message = U;

    crate::impl_view_with_inner!(inner);

    #[inline]
    fn on_event(
        &mut self,
        state: &mut S,
        e: E,
    ) -> Option<U> {
        let msg = self.inner.on_event(state, e);

        msg.map(|msg| (self.f)(&mut self.inner, state, msg))
    }
}

pub struct MapE<H, NE, F> {
    inner:   H,
    f:       F,
    _marker: PhantomData<NE>,
}

impl<H, NE, F> MapE<H, NE, F> {
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

impl<S, H, E, NE, F> View<S, E> for MapE<H, NE, F>
where
    H: View<S, NE>,
    F: FnMut(&mut H, &mut S, E) -> NE,
{
    type Message = H::Message;

    crate::impl_view_with_inner!(inner);

    #[inline]
    fn on_event(
        &mut self,
        state: &mut S,
        e: E,
    ) -> Option<Self::Message> {
        let e = (self.f)(&mut self.inner, state, e);
        self.inner.on_event(state, e)
    }
}

pub struct MapOptE<H, NE, F> {
    inner:   H,
    f:       F,
    _marker: PhantomData<NE>,
}

impl<H, NE, F> MapOptE<H, NE, F> {
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

impl<S, H, E, NE, F> View<S, NE> for MapOptE<H, NE, F>
where
    H: View<S, E>,
    F: FnMut(&mut H, &mut S, NE) -> Option<E>,
{
    type Message = H::Message;

    crate::impl_view_with_inner!(inner);

    #[inline]
    fn on_event(
        &mut self,
        state: &mut S,
        e: NE,
    ) -> Option<Self::Message> {
        let e = (self.f)(&mut self.inner, state, e);

        match e {
            Some(e) => self.inner.on_event(state, e),
            None => None,
        }
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

impl<S, E, H, F> View<S, E> for OrElseFirst<H, F>
where
    E: Clone,
    H: View<S, E>,
    F: FnMut(&mut H, &mut S, E) -> Option<H::Message>,
{
    type Message = H::Message;

    crate::impl_view_with_inner!(inner);

    #[inline]
    fn on_event(
        &mut self,
        state: &mut S,
        e: E,
    ) -> Option<Self::Message> {
        (self.f)(&mut self.inner, state, e.clone()).or_else(|| self.inner.on_event(state, e))
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

impl<S, E, H, F> View<S, E> for OrElse<H, F>
where
    E: Clone,
    H: View<S, E>,
    F: FnMut(&mut H, &mut S, E) -> Option<H::Message>,
{
    type Message = H::Message;

    crate::impl_view_with_inner!(inner);

    #[inline]
    fn on_event(
        &mut self,
        state: &mut S,
        e: E,
    ) -> Option<Self::Message> {
        self.inner
            .on_event(state, e.clone())
            .or_else(|| (self.f)(&mut self.inner, state, e))
    }
}
