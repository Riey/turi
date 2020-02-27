use crate::{
    event::{
        EventLike,
        KeyEventLike,
        MouseEventLike,
    },
    orientation::Orientation,
    printer::Printer,
    rect::Rect,
    state::RedrawState,
    vec2::Vec2,
    view::{
        ScrollableView,
        View,
    },
};
use std::cell::Cell;

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

impl<S, E, T, M> View<S, E> for ConsumeEvent<T, M>
where
    M: Clone,
    T: View<S, E>,
{
    type Message = M;

    impl_view_with_inner!(inner);

    #[inline]
    fn on_event(
        &mut self,
        _state: &mut S,
        _event: E,
    ) -> Option<Self::Message> {
        Some(self.msg.clone())
    }
}

impl<S, E, T, M> ScrollableView<S, E> for ConsumeEvent<T, M>
where
    T: ScrollableView<S, E>,
    M: Clone,
{
    impl_scrollable_view_with_inner!(inner);
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

impl<S: RedrawState, E: EventLike, T> View<S, E> for ScrollView<T>
where
    T: ScrollableView<S, E>,
{
    type Message = T::Message;

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

    fn on_event(
        &mut self,
        state: &mut S,
        event: E,
    ) -> Option<Self::Message> {
        // TODO: check focus

        if let Some(me) = event.try_mouse() {
            if me.try_left_up().is_some() {
                self.clicked = false;
            } else if let Some(pos) = me.try_left_down() {
                if self.event_mouse_down(pos, state) {
                    return None;
                }
            } else if let Some(pos) = me.try_drag() {
                if self.event_drag(pos, state) {
                    return None;
                }
            }
        } else if let Some(ke) = event.try_key() {
            if ke.try_up() && self.orientation == Orientation::Vertical
                || ke.try_left() && self.orientation == Orientation::Horizontal
            {
                if self.down() {
                    state.set_need_redraw(true);
                }
                return None;
            } else if ke.try_down() && self.orientation == Orientation::Vertical
                || ke.try_right() && self.orientation == Orientation::Horizontal
            {
                if self.up() {
                    state.set_need_redraw(true);
                }
                return None;
            }
        }
        self.inner.on_event(state, event)
    }
}

pub struct SizeCacher<T> {
    inner:     T,
    prev_size: Vec2,
}

impl<T> SizeCacher<T> {
    #[inline]
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

impl<S, E, T> View<S, E> for SizeCacher<T>
where
    T: View<S, E>,
{
    type Message = T::Message;

    #[inline]
    fn render(
        &self,
        printer: &mut Printer,
    ) {
        self.inner.render(printer);
    }

    #[inline]
    fn layout(
        &mut self,
        size: Vec2,
    ) {
        self.prev_size = size;
        self.inner.layout(size);
    }

    #[inline]
    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size()
    }

    #[inline]
    fn on_event(
        &mut self,
        state: &mut S,
        event: E,
    ) -> Option<Self::Message> {
        self.inner.on_event(state, event)
    }
}

impl_scrollable_view_for_inner!(SizeCacher<T>);
