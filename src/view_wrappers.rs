use crate::{
    event::{
        EventLike,
        KeyEventLike,
        MouseEventLike,
    },
    orientation::Orientation,
    printer::Printer,
    state::RedrawState,
    vec2::Vec2,
    view::View,
};
use std::cell::Cell;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FullSizeStrategy {
    LeftTop,
    Centered,
}

impl FullSizeStrategy {
    fn calc_x(
        self,
        inner_width: u16,
        printer_width: u16,
    ) -> u16 {
        match self {
            FullSizeStrategy::LeftTop => 0,
            FullSizeStrategy::Centered => {
                match printer_width.checked_sub(inner_width) {
                    Some(diff) => diff / 2,
                    _ => 0,
                }
            }
        }
    }

    fn calc_y(
        self,
        inner_height: u16,
        printer_height: u16,
    ) -> u16 {
        match self {
            FullSizeStrategy::LeftTop => 0,
            FullSizeStrategy::Centered => {
                match printer_height.checked_sub(inner_height) {
                    Some(diff) => diff / 2,
                    _ => 0,
                }
            }
        }
    }

    pub fn calc_margin(
        self,
        inner_size: Vec2,
        printer_size: Vec2,
    ) -> Vec2 {
        Vec2::new(
            self.calc_x(inner_size.x, printer_size.x),
            self.calc_y(inner_size.y, printer_size.y),
        )
    }
}

pub struct FullSizeView<T> {
    inner:       T,
    prev_margin: Cell<Vec2>,
    strategy:    FullSizeStrategy,
}

impl<T> FullSizeView<T> {
    pub fn new(
        inner: T,
        strategy: FullSizeStrategy,
    ) -> Self {
        Self {
            inner,
            prev_margin: Cell::new(Vec2::new(0, 0)),
            strategy,
        }
    }
}

impl<S, E: EventLike, T> View<S, E> for FullSizeView<T>
where
    T: View<S, E>,
{
    type Message = T::Message;

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        let margin = self
            .strategy
            .calc_margin(self.inner.desired_size(), printer.bound().size());

        self.prev_margin.set(margin);

        printer.with_bound(printer.bound().add_start(margin), |printer| {
            self.inner.render(printer);
        });
    }

    fn layout(
        &mut self,
        size: Vec2,
    ) {
        self.inner.layout(size);
    }

    fn desired_size(&self) -> Vec2 {
        Vec2::new(std::u16::MAX, std::u16::MAX)
    }

    fn on_event(
        &mut self,
        state: &mut S,
        mut event: E,
    ) -> Option<Self::Message> {
        if let Some(me) = event.try_mouse_mut() {
            if me.filter_map_pos(|pos| {
                let margin = self.prev_margin.get();
                if pos >= margin {
                    Some(pos - margin)
                } else {
                    None
                }
            }) {
                return self.inner.on_event(state, event);
            }

            None
        } else {
            self.inner.on_event(state, event)
        }
    }
}

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

    crate::impl_view_with_inner!(inner);

    #[inline]
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
        match self.orientation {
            Orientation::Vertical => {
                if pos.x == self.inner.prev_size().x {
                    self.clicked = true;
                    self.scroll = pos.y;
                    state.set_need_redraw(true);
                    true
                } else {
                    false
                }
            }
            Orientation::Horizontal => {
                if pos.y == self.inner.prev_size().y {
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
    T: View<S, E>,
{
    type Message = T::Message;

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        let inner_size = self.inner.desired_size();

        let pos = match self.orientation {
            Orientation::Horizontal => {
                match inner_size.x.checked_sub(printer.bound().w()) {
                    Some(left) => left * self.scroll / (printer.bound().w() - 1),
                    None => {
                        // Nothing to scroll
                        self.inner.render(printer);
                        return;
                    }
                }
            }
            Orientation::Vertical => {
                match inner_size.y.checked_sub(printer.bound().h()) {
                    Some(left) => left * self.scroll / (printer.bound().h() - 1),
                    None => {
                        // Nothing to scroll
                        self.inner.render(printer);
                        return;
                    }
                }
            }
        };

        printer.with_bound(
            printer.bound().sub_size(self.additional_size()),
            |printer| {
                let pos = match self.orientation {
                    Orientation::Horizontal => (pos, 0),
                    Orientation::Vertical => (0, pos),
                };
                printer.sliced(pos, |printer| {
                    self.inner.render(printer);
                });
            },
        );

        match self.orientation {
            Orientation::Horizontal => {
                let pos = printer.bound().h() - 1;
                printer.print_horizontal_line(pos);
                printer.print((self.scroll, pos), self.scroll_block_text());
            }
            Orientation::Vertical => {
                let pos = printer.bound().w() - 1;
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
            } else if let Some(pos) = me.try_scroll_up() {
                let inner_size = self.inner.prev_size();
                match self.orientation {
                    Orientation::Vertical if self.scroll < inner_size.x => {
                        if self.up() {
                            state.set_need_redraw(true);
                        }
                        return None;
                    }
                    Orientation::Horizontal if pos.y == inner_size.y => {
                        if self.up() {
                            state.set_need_redraw(true);
                        }
                        return None;
                    }
                    _ => {}
                }
            } else if let Some(pos) = me.try_scroll_down() {
                let inner_size = self.inner.prev_size();
                match self.orientation {
                    Orientation::Vertical if self.scroll < inner_size.x => {
                        if self.down() {
                            state.set_need_redraw(true);
                        }
                        return None;
                    }
                    Orientation::Horizontal if pos.y == inner_size.y => {
                        if self.down() {
                            state.set_need_redraw(true);
                        }
                        return None;
                    }
                    _ => {}
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
