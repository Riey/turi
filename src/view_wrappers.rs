use crate::{
    event::{
        EventHandler,
        EventLike,
    },
    orientation::Orientation,
    printer::Printer,
    rect::Rect,
    vec2::Vec2,
    view::{
        ScrollableView,
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

impl_deref_for_inner!(ConsumeEvent<T, M>);
impl_deref_for_inner!(ScrollView<T>);
impl_deref_for_inner!(EventMarker<T, E>);
impl_deref_for_inner!(SizeCacher<T>);
impl_deref_for_inner!(BoundChecker<T>);

pub struct ConsumeEvent<T, M> {
    inner: T,
    msg:   M,
}

impl<T, M> ConsumeEvent<T, M> {
    pub fn new(
        inner: T,
        msg: M,
    ) -> Self {
        Self { inner, msg }
    }
}

impl<T, M> ViewProxy for ConsumeEvent<T, M>
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

impl<S, E, T, M> EventHandler<S, E> for ConsumeEvent<T, M>
where
    M: Clone,
{
    type Message = M;

    #[inline(always)]
    fn on_event(
        &mut self,
        _state: &mut S,
        _event: E,
    ) -> Option<Self::Message> {
        Some(self.msg.clone())
    }
}

pub struct ScrollView<T> {
    inner:       SizeCacher<T>,
    orientation: Orientation,
    scroll:      u16,
}

impl<T> ScrollView<T> {
    pub fn new(
        inner: T,
        orientation: Orientation,
    ) -> Self {
        Self {
            inner: SizeCacher::new(inner),
            orientation,
            scroll: 0,
        }
    }

    #[inline]
    fn down(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    #[inline]
    fn up(&mut self) {
        self.scroll = (self.scroll + 1).min(match self.orientation {
            Orientation::Vertical => self.inner.prev_size.y,
            Orientation::Horizontal => self.inner.prev_size.x,
        });
    }

    #[inline]
    fn additional_size(&self) -> Vec2 {
        match self.orientation {
            Orientation::Horizontal => Vec2::new(0, 1),
            Orientation::Vertical => Vec2::new(1, 0),
        }
    }
}

impl<T> View for ScrollView<T>
where
    T: ScrollableView,
{
    fn render(
        &self,
        printer: &mut Printer,
    ) {
        printer.with_bound(
            printer.bound().sub_size(self.additional_size()),
            |printer| {
                match self.orientation {
                    Orientation::Horizontal => {
                        self.inner.scroll_horizontal_render(self.scroll, printer);
                    }
                    Orientation::Vertical => {
                        self.inner.scroll_vertical_render(self.scroll, printer);
                    }
                }
            },
        );

        match self.orientation {
            Orientation::Horizontal => {
                let pos = printer.bound().y() + printer.bound().h();
                printer.print_horizontal_line(pos);
                printer.print_horizontal_block_line_at((self.scroll, pos), 4);
            }
            Orientation::Vertical => {
                let pos = printer.bound().x() + printer.bound().w();
                printer.print_vertical_line(pos);
                printer.print_vertical_block_line_at((pos, self.scroll), 4);
            }
        }
    }

    fn layout(
        &mut self,
        mut size: Vec2,
    ) {
        size = size.saturating_sub(self.additional_size());
        self.inner.layout(size);
    }

    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size() + self.additional_size()
    }
}

impl<S, E: EventLike, T> EventHandler<S, E> for ScrollView<T>
where
    T: EventHandler<S, E>,
{
    type Message = T::Message;

    fn on_event(
        &mut self,
        state: &mut S,
        event: E,
    ) -> Option<Self::Message> {
        // TODO: check click, focus

        match self.orientation {
            Orientation::Vertical => {
                if event.try_up() {
                    self.down();
                    return None;
                } else if event.try_down() {
                    self.up();
                    return None;
                }
            }
            Orientation::Horizontal => {
                if event.try_left() {
                    self.down();
                    return None;
                } else if event.try_right() {
                    self.up();
                    return None;
                }
            }
        }

        self.inner.on_event(state, event)
    }
}

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
