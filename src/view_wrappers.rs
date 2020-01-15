use crate::rect::Rect;
use crate::{
    printer::Printer,
    vec2::Vec2,
    view::{View, },
};
use crossterm::event::{Event, MouseEvent};
use std::cell::Cell;

impl_deref_for_generic_inner!(SizeCacher => inner);
impl_deref_for_generic_inner!(BoundChecker => inner);

pub struct SizeCacher<T> {
    inner: T,
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

impl<S, T> View<S> for SizeCacher<T>
where
    T: View<S>,
{
    type Message = T::Message;

    fn render(&self, printer: &mut Printer) {
        self.inner.render(printer);
    }

    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size()
    }

    fn layout(&mut self, size: Vec2) {
        self.prev_size = size;
        self.inner.layout(size);
    }

    fn on_event(&mut self, state: &mut S, e: Event) -> Option<T::Message> {
        self.inner.on_event(state, e)
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
        self.contains(get_pos_from_me(me))
    }
}

impl<S, T> View<S> for BoundChecker<T>
where
    T: View<S>,
{
    type Message = T::Message;

    fn render(&self, printer: &mut Printer) {
        self.bound.set(printer.bound());
        self.inner.render(printer);
    }

    fn desired_size(&self) -> Vec2 {
        self.inner.desired_size()
    }

    fn layout(&mut self, size: Vec2) {
        self.inner.layout(size);
    }

    fn on_event(&mut self, state: &mut S, e: Event) -> Option<T::Message> {
        self.inner.on_event(state, e)
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
