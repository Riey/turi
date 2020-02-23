use crate::{
    event::EventHandler,
    printer::Printer,
    rect::Rect,
    vec2::Vec2,
    view::{
        View,
        ViewProxy,
    },
};
use std::{
    cell::Cell,
    marker::PhantomData,
    ops::{
        Deref,
        DerefMut,
    },
};

macro_rules! impl_deref_for_inner {
    ($ident:ident<$inner:ident $(,$gen:ident)*>) => {
        impl<$inner $(,$gen)*> Deref for $ident<$inner $(,$gen)*> {
            type Target = $inner;

            #[inline(always)]
            fn deref(&self) -> &$inner {
                &self.inner
            }
        }
        impl<$inner $(,$gen)*> DerefMut for $ident<$inner $(,$gen)*> {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut $inner {
                &mut self.inner
            }
        }

    };
}

impl_deref_for_inner!(EventMarker<T, E>);
impl_deref_for_inner!(SizeCacher<T>);
impl_deref_for_inner!(BoundChecker<T>);

pub struct EventMarker<T, E> {
    inner:   T,
    _marker: PhantomData<E>,
}

impl<T, E> EventMarker<T, E> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<T, E> ViewProxy for EventMarker<T, E>
where
    T: View,
{
    type Inner = T;

    #[inline(always)]
    fn get_inner(&self) -> &Self::Inner {
        &self.inner
    }

    #[inline(always)]
    fn get_inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
}

impl<S, T, E> EventHandler<S, E> for EventMarker<T, E>
where
    T: EventHandler<S, E>,
{
    type Message = T::Message;

    #[inline(always)]
    fn on_event(
        &mut self,
        state: &mut S,
        event: E,
    ) -> Option<Self::Message> {
        self.inner.on_event(state, event)
    }
}

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

impl<T> ViewProxy for SizeCacher<T>
where
    T: View,
{
    type Inner = T;

    fn get_inner(&self) -> &T {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    #[inline(always)]
    fn proxy_layout(
        &mut self,
        size: Vec2,
    ) {
        self.prev_size = size;
        self.inner.layout(size);
    }
}

impl<S, E, T> EventHandler<S, E> for SizeCacher<T>
where
    T: EventHandler<S, E>,
{
    type Message = T::Message;

    #[inline(always)]
    fn on_event(
        &mut self,
        state: &mut S,
        event: E,
    ) -> Option<Self::Message> {
        self.inner.on_event(state, event)
    }
}

pub struct BoundChecker<T> {
    inner: T,
    bound: Cell<Rect>,
}

impl<T> BoundChecker<T> {
    #[inline(always)]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            bound: Cell::new(Rect::new((0, 0), (0, 0))),
        }
    }

    #[inline(always)]
    pub fn contains(
        &self,
        p: Vec2,
    ) -> bool {
        self.bound.get().contains(p)
    }
}

impl<T> ViewProxy for BoundChecker<T>
where
    T: View,
{
    type Inner = T;

    #[inline(always)]
    fn get_inner(&self) -> &T {
        &self.inner
    }

    #[inline(always)]
    fn get_inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    #[inline(always)]
    fn proxy_render(
        &self,
        printer: &mut Printer,
    ) {
        self.bound.set(printer.bound());
        self.inner.render(printer);
    }
}

impl<S, E, T> EventHandler<S, E> for BoundChecker<T>
where
    T: EventHandler<S, E>,
{
    type Message = T::Message;

    #[inline(always)]
    fn on_event(
        &mut self,
        state: &mut S,
        event: E,
    ) -> Option<Self::Message> {
        self.inner.on_event(state, event)
    }
}
