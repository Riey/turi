use crate::{
    attribute::Attribute,
    event::{
        EventLike,
        KeyEventLike,
        MouseEventLike,
    },
    printer::Printer,
    vec2::Vec2,
};
use unicode_width::UnicodeWidthStr;

impl<'a, E, M> Clone for View<'a, E, M> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, E, M> Copy for View<'a, E, M> {}

impl<'a, E, M> Clone for ViewInner<'a, E, M> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, E, M> Copy for ViewInner<'a, E, M> {}

#[derive(Clone, Copy, Debug)]
pub enum Tag {
    Div,
    Button,
}

pub enum ViewInner<'a, E, M> {
    Text(&'a str),
    Children(&'a [View<'a, E, M>]),
}

pub struct View<'a, E, M> {
    tag:   Tag,
    attr:  Attribute<'a, E, M>,
    inner: ViewInner<'a, E, M>,
}

impl<'a, E, M> View<'a, E, M> {
    pub fn new(
        tag: Tag,
        attr: Attribute<'a, E, M>,
        inner: ViewInner<'a, E, M>,
    ) -> Self {
        Self { tag, attr, inner }
    }

    pub fn tag(self) -> Tag {
        self.tag
    }

    pub fn render(
        self,
        printer: &mut Printer,
    ) {
        todo!()
    }

    pub fn desired_size(self) -> Vec2 {
        Vec2::new(1, 1)
    }

    pub fn on_event(
        self,
        e: E,
    ) -> Option<M> {
        None
    }
}
