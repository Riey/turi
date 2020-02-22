use crate::view::ViewProxy;
use crate::{
    printer::Printer,
    rect::Rect,
    vec2::Vec2,
    view::View,
};
use std::cell::Cell;

impl_deref_for_generic_inner!(SizeCacher => inner);
impl_deref_for_generic_inner!(BoundChecker => inner);

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

    fn get_inner(&self) -> &T { &self.inner }
    fn get_inner_mut(&mut self) -> &mut T { &mut self.inner }

    fn proxy_layout(
        &mut self,
        size: Vec2,
    ) {
        self.prev_size = size;
        self.inner.layout(size);
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

    fn get_inner(&self) -> &T { &self.inner }
    fn get_inner_mut(&mut self) -> &mut T { &mut self.inner }

    fn proxy_render(
        &self,
        printer: &mut Printer,
    ) {
        self.bound.set(printer.bound());
        self.inner.render(printer);
    }
}
