use crate::{
    event::EventHandler,
    view::{
        View,
        ViewProxy,
    },
};
use std::{
    marker::PhantomData,
    ops::{
        Deref,
        DerefMut,
    },
};

macro_rules! impl_view_proxy {
    ($ident:ident<$inner:ident $(,$gen:ident)+>) => {
        impl<$inner $(,$gen)+> ViewProxy for $ident<$inner $(,$gen)+> where $inner: View {
            type Inner = $inner;

            #[inline(always)]
            fn get_inner(&self) -> &$inner {
                &self.inner
            }

            #[inline(always)]
            fn get_inner_mut(&mut self) -> &mut $inner {
                &mut self.inner
            }
        }

        impl<$inner $(,$gen)+> Deref for $ident<$inner $(,$gen)+> {
            type Target = $inner;

            #[inline(always)]
            fn deref(&self) -> &$inner {
                &self.inner
            }
        }
        impl<$inner $(,$gen)+> DerefMut for $ident<$inner $(,$gen)+> {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut $inner {
                &mut self.inner
            }
        }

    };
}

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

impl<S, E, H, U, F> EventHandler<S, E> for Map<H, U, F>
where
    H: EventHandler<S, E>,
    F: FnMut(&mut H, &mut S, H::Message) -> U,
{
    type Message = U;

    #[inline(always)]
    fn on_event(
        &mut self,
        state: &mut S,
        e: E,
    ) -> Option<U> {
        let msg = self.inner.on_event(state, e);

        msg.map(|msg| (self.f)(&mut self.inner, state, msg))
    }
}

impl_view_proxy!(Map<H, U, F>);

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

impl<S, H, E, NE, F> EventHandler<S, E> for MapE<H, NE, F>
where
    H: EventHandler<S, NE>,
    F: FnMut(&mut H, &mut S, E) -> NE,
{
    type Message = H::Message;

    #[inline(always)]
    fn on_event(
        &mut self,
        state: &mut S,
        e: E,
    ) -> Option<Self::Message> {
        let e = (self.f)(&mut self.inner, state, e);
        self.inner.on_event(state, e)
    }
}

impl_view_proxy!(MapE<H, NE, F>);

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

impl<S, H, E, NE, F> EventHandler<S, NE> for MapOptE<H, NE, F>
where
    H: EventHandler<S, E>,
    F: FnMut(&mut H, &mut S, NE) -> Option<E>,
{
    type Message = H::Message;

    #[inline(always)]
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

impl_view_proxy!(MapOptE<H, NE, F>);

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

impl<S, E, H, F> EventHandler<S, E> for OrElseFirst<H, F>
where
    E: Clone,
    H: EventHandler<S, E>,
    F: FnMut(&mut H, &mut S, E) -> Option<H::Message>,
{
    type Message = H::Message;

    #[inline(always)]
    fn on_event(
        &mut self,
        state: &mut S,
        e: E,
    ) -> Option<Self::Message> {
        (self.f)(&mut self.inner, state, e.clone()).or_else(|| self.inner.on_event(state, e))
    }
}

impl_view_proxy!(OrElseFirst<H, F>);

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

impl<S, E, H, F> EventHandler<S, E> for OrElse<H, F>
where
    E: Clone,
    H: EventHandler<S, E>,
    F: FnMut(&mut H, &mut S, E) -> Option<H::Message>,
{
    type Message = H::Message;

    #[inline(always)]
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

impl_view_proxy!(OrElse<H, F>);
