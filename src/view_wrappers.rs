use crate::{
    event::{
        EventLike,
        KeyEventLike,
        MouseEventLike,
    },
    orientation::Orientation,
    printer::Printer,
    try_consume,
    vec2::Vec2,
    view::{
        EventResult,
        View,
        IGNORE,
        REDRAW,
    },
};

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
        if self.scroll != 0 {
            true
        } else {
            false
        }
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
    ) -> EventResult {
        match self.orientation {
            Orientation::Vertical => {
                if pos.x == self.inner.prev_size().x {
                    self.clicked = true;
                    self.scroll = pos.y;
                    REDRAW
                } else {
                    IGNORE
                }
            }
            Orientation::Horizontal => {
                if pos.y == self.inner.prev_size().y {
                    self.clicked = true;
                    self.scroll = pos.x;
                    REDRAW
                } else {
                    IGNORE
                }
            }
        }
    }

    #[inline]
    fn event_drag(
        &mut self,
        pos: Vec2,
    ) -> EventResult {
        if self.clicked {
            self.scroll = match self.orientation {
                Orientation::Vertical => pos.y,
                Orientation::Horizontal => pos.x,
            };
            REDRAW
        } else {
            IGNORE
        }
    }
}

impl<S, E: EventLike, T> View<S, E> for ScrollView<T>
where
    T: View<S, E>,
{
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
    ) -> EventResult {
        // TODO: check focus

        if let Some(me) = event.try_mouse() {
            if me.try_left_up().is_some() {
                self.clicked = false;
            } else if let Some(pos) = me.try_left_down() {
                try_consume!(self.event_mouse_down(pos));
            } else if let Some(pos) = me.try_drag() {
                try_consume!(self.event_drag(pos));
            } else if let Some(pos) = me.try_scroll_up() {
                let inner_size = self.inner.prev_size();
                match self.orientation {
                    Orientation::Vertical if self.scroll < inner_size.x => {
                        return EventResult::Consume(self.up());
                    }
                    Orientation::Horizontal if pos.y == inner_size.y => {
                        return EventResult::Consume(self.up());
                    }
                    _ => {}
                }
            } else if let Some(pos) = me.try_scroll_down() {
                let inner_size = self.inner.prev_size();
                match self.orientation {
                    Orientation::Vertical if self.scroll < inner_size.x => {
                        return EventResult::Consume(self.down());
                    }
                    Orientation::Horizontal if pos.y == inner_size.y => {
                        return EventResult::Consume(self.down());
                    }
                    _ => {}
                }
            }
        } else if let Some(ke) = event.try_key() {
            if ke.try_up() && self.orientation == Orientation::Vertical
                || ke.try_left() && self.orientation == Orientation::Horizontal
            {
                return EventResult::Consume(self.down());
            } else if ke.try_down() && self.orientation == Orientation::Vertical
                || ke.try_right() && self.orientation == Orientation::Horizontal
            {
                return EventResult::Consume(self.up());
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
    ) -> EventResult {
        self.inner.on_event(state, event)
    }
}
