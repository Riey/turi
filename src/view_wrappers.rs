use crate::rect::Rect;
use crate::{
    printer::Printer,
    vec2::Vec2,
    view::{View, ViewProxy},
};
use crossterm::event::{Event, MouseEvent};
use std::cell::Cell;

pub struct SizeCacher<T> {
    inner: T,
    prev_size: Cell<Vec2>,
}

impl<T> SizeCacher<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            prev_size: Cell::new(Vec2::new(0, 0)),
        }
    }

    #[inline]
    pub fn prev_size(&self) -> Vec2 {
        self.prev_size.get()
    }
}

impl<T> ViewProxy for SizeCacher<T>
where
    T: View,
{
    type Inner = T;
    type Message = T::Message;

    fn inner_view(&self) -> &T {
        &self.inner
    }
    fn inner_view_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    fn proxy_desired_size(&self) -> Vec2 {
        // TODO: move this to layout
        self.prev_size.set(self.inner.desired_size());
        self.prev_size()
    }

    fn proxy_on_event(&mut self, e: Event) -> Option<T::Message> {
        self.inner_view_mut().on_event(e)
    }
}

pub struct BoundChecker<T> {
    inner: T,
    bound: Cell<Rect>,
}

impl<T> BoundChecker<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            bound: Cell::new(Rect::new((0, 0), (0, 0))),
        }
    }

    pub fn contains(&self, p: Vec2) -> bool {
        self.bound.get().contains(p)
    }

    pub fn contains_cursor(&self, me: MouseEvent) -> bool {
        log::trace!(
            "check contains_cursor from {:?} in {:?}",
            get_pos_from_me(me),
            self.bound
        );
        self.contains(get_pos_from_me(me))
    }
}

impl<T> ViewProxy for BoundChecker<T>
where
    T: View,
{
    type Inner = T;
    type Message = T::Message;

    fn inner_view(&self) -> &T {
        &self.inner
    }
    fn inner_view_mut(&mut self) -> &mut T {
        &mut self.inner
    }
    fn proxy_render(&self, printer: &mut Printer) {
        self.bound.set(printer.bound());
        self.inner.render(printer);
    }
    fn proxy_on_event(&mut self, e: Event) -> Option<T::Message> {
        self.inner.on_event(e)
    }
}

fn get_pos_from_me(me: MouseEvent) -> Vec2 {
    match me {
        MouseEvent::Up(_, x, y, _)
        | MouseEvent::Down(_, x, y, _)
        | MouseEvent::Drag(_, x, y, _)
        | MouseEvent::ScrollUp(x, y, _)
        | MouseEvent::ScrollDown(x, y, _) => Vec2::new(x, y),
    }
}
