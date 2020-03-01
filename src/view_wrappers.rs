use crate::{
    event::{
        Event,
        KeyCode,
        KeyEvent,
        KeyEventType,
        MouseButton,
        MouseEvent,
    },
    orientation::Orientation,
    printer::Printer,
    state::RedrawState,
    vec2::Vec2,
    view::View,
};

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

impl<S, T, M> View<S> for ConsumeEvent<T, M>
where
    M: Clone,
    T: View<S>,
{
    type Message = M;

    crate::impl_view_with_inner!(inner);

    #[inline]
    fn on_event(
        &mut self,
        _state: &mut S,
        _event: Event,
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

impl<S: RedrawState, T> View<S> for ScrollView<T>
where
    T: View<S>,
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
        event: Event,
    ) -> Option<Self::Message> {
        // TODO: check focus

        match event {
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Down(MouseButton::Left, pos) => {
                        if self.event_mouse_down(pos, state) {
                            return None;
                        }
                    }
                    MouseEvent::Drag(MouseButton::Left, pos) => {
                        if self.event_drag(pos, state) {
                            return None;
                        }
                    }
                    MouseEvent::Up(MouseButton::Left, _) => {
                        self.clicked = false;
                    }
                    _ => {}
                }
            }
            Event::Key(KeyEvent(ty, modifiers)) => {
                if modifiers.is_empty() {
                    let (up, down) = match self.orientation {
                        Orientation::Vertical => (KeyCode::Down, KeyCode::Up),
                        Orientation::Horizontal => (KeyCode::Right, KeyCode::Left),
                    };
                    match ty {
                        KeyEventType::Key(code) if code == down => {
                            if self.down() {
                                state.set_need_redraw(true);
                            }
                            return None;
                        }
                        KeyEventType::Key(code) if code == up => {
                            if self.up() {
                                state.set_need_redraw(true);
                            }
                            return None;
                        }
                        _ => {}
                    }
                }
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

impl<S, T> View<S> for SizeCacher<T>
where
    T: View<S>,
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
        event: Event,
    ) -> Option<Self::Message> {
        self.inner.on_event(state, event)
    }
}
