use crate::{
    event::{
        EventHandler,
        EventLike,
    },
    orientation::Orientation,
    printer::Printer,
    rect::Rect,
    state::RedrawState,
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
    clicked:     bool,
    prev_bound:  Cell<Rect>,
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
            clicked: false,
            prev_bound: Cell::new(Rect::new((0, 0), (0, 0))),
        }
    }

    #[inline]
    fn down(&mut self) -> bool {
        self.scroll = self.scroll.saturating_sub(1);
        self.scroll != 0
    }

    #[inline]
    fn up(&mut self) -> bool {
        let can_up = self.scroll + 1
            < match self.orientation {
                Orientation::Vertical => self.inner.prev_size.y,
                Orientation::Horizontal => self.inner.prev_size.x,
            };

        if can_up {
            self.scroll += 1;
            true
        } else {
            false
        }
    }

    #[inline]
    fn additional_size(&self) -> Vec2 {
        match self.orientation {
            Orientation::Horizontal => Vec2::new(0, 1),
            Orientation::Vertical => Vec2::new(1, 0),
        }
    }

    #[inline]
    fn scroll_block_text(&self) -> &'static str {
        if self.clicked {
            "█"
        } else {
            "░"
        }
    }

    #[inline]
    fn event_mouse_down(
        &mut self,
        pos: Vec2,
        state: &mut impl RedrawState,
    ) -> bool {
        let prev_bound = self.prev_bound.get();
        let pos = pos - prev_bound.start();

        match self.orientation {
            Orientation::Vertical => {
                if pos.x + 1 == prev_bound.x() + prev_bound.w() {
                    self.clicked = true;
                    self.scroll = pos.y;
                    state.set_need_redraw(true);
                    true
                } else {
                    false
                }
            }
            Orientation::Horizontal => {
                if pos.y + 1 == prev_bound.y() + prev_bound.h() {
                    self.clicked = true;
                    self.scroll = pos.x;
                    state.set_need_redraw(true);
                    true
                } else {
                    false
                }
            }
        }
    }

    #[inline]
    fn event_drag(
        &mut self,
        pos: Vec2,
        state: &mut impl RedrawState,
    ) -> bool {
        if self.clicked {
            let prev_bound = self.prev_bound.get();
            let pos = pos - prev_bound.start();
            self.scroll = match self.orientation {
                Orientation::Vertical => pos.y,
                Orientation::Horizontal => pos.x,
            };
            state.set_need_redraw(true);
            true
        } else {
            false
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
        self.prev_bound.set(printer.bound());

        let inner_size = self.inner.desired_size();

        let pos = match self.orientation {
            Orientation::Horizontal => {
                match inner_size.x.checked_sub(printer.bound().w()) {
                    Some(left) => left * self.scroll / (printer.bound().w() - 1),
                    None => {
                        self.inner.render(printer);
                        return;
                    }
                }
            }
            Orientation::Vertical => {
                match inner_size.y.checked_sub(printer.bound().h()) {
                    Some(left) => left * self.scroll / (printer.bound().h() - 1),
                    None => {
                        self.inner.render(printer);
                        return;
                    }
                }
            }
        };

        printer.with_bound(
            printer.bound().sub_size(self.additional_size()),
            |printer| {
                match self.orientation {
                    Orientation::Horizontal => {
                        self.inner.scroll_horizontal_render(pos, printer);
                    }
                    Orientation::Vertical => {
                        self.inner.scroll_vertical_render(pos, printer);
                    }
                }
            },
        );

        match self.orientation {
            Orientation::Horizontal => {
                let pos = printer.bound().y() + printer.bound().h();
                printer.print_horizontal_line(pos);
                printer.print((self.scroll, pos), self.scroll_block_text());
            }
            Orientation::Vertical => {
                let pos = printer.bound().x() + printer.bound().w();
                printer.print_vertical_line(pos);
                printer.print((pos, self.scroll), self.scroll_block_text());
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

impl<S: RedrawState, E: EventLike, T> EventHandler<S, E> for ScrollView<T>
where
    T: EventHandler<S, E>,
{
    type Message = T::Message;

    fn on_event(
        &mut self,
        state: &mut S,
        event: E,
    ) -> Option<Self::Message> {
        // TODO: check focus

        if event.try_mouse_up().is_some() {
            self.clicked = false;
        } else if let Some(pos) = event.try_mouse_down() {
            if self.event_mouse_down(pos, state) {
                return None;
            }
        } else if let Some(pos) = event.try_drag() {
            if self.event_drag(pos, state) {
                return None;
            }
        } else if event.try_up() && self.orientation == Orientation::Vertical
            || event.try_left() && self.orientation == Orientation::Horizontal
        {
            if self.down() {
                state.set_need_redraw(true);
            }
            return None;
        } else if event.try_down() && self.orientation == Orientation::Vertical
            || event.try_right() && self.orientation == Orientation::Horizontal
        {
            if self.up() {
                state.set_need_redraw(true);
            }
            return None;
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

    #[inline(always)]
    fn get_inner(&self) -> &T {
        &self.inner
    }

    #[inline(always)]
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
