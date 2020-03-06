use crate::{
    event::EventLike,
    event_filter::EventFilter,
    view::{
        Tag,
        View,
        ViewBody,
    },
};

use bumpalo::{
    collections::Vec,
    Bump,
};
use unicode_width::UnicodeWidthStr;

pub struct ClassBuilder<'a> {
    classes: Vec<'a, &'a str>,
}

impl<'a> ClassBuilder<'a> {
    pub fn new(b: &'a Bump) -> Self {
        Self {
            classes: Vec::with_capacity_in(3, b),
        }
    }

    pub fn class(
        mut self,
        class: &'a str,
    ) -> Self {
        self.classes.push(class);
        self
    }
}

pub struct EventBuilder<'a, E, M> {
    b:      &'a Bump,
    events: Vec<'a, EventFilter<'a, E, M>>,
}

impl<'a, E, M> EventBuilder<'a, E, M> {
    pub fn new(b: &'a Bump) -> Self {
        Self {
            b,
            events: Vec::with_capacity_in(3, b),
        }
    }

    pub fn ctrl_char(
        mut self,
        ch: char,
        msg: M,
    ) -> Self
    where
        E: EventLike,
        M: Copy,
    {
        self.events.push(EventFilter::ctrl_char(self.b, ch, msg));
        self
    }
}

pub struct BodyBuilder<'a, E, M> {
    children: Vec<'a, View<'a, E, M>>,
}

impl<'a, E, M> BodyBuilder<'a, E, M> {
    pub fn new(b: &'a Bump) -> Self {
        Self {
            children: Vec::with_capacity_in(3, b),
        }
    }

    pub fn child(
        mut self,
        child: View<'a, E, M>,
    ) -> Self {
        self.children.push(child);
        self
    }
}

impl<'a, E, M> Builder<ViewBody<'a, E, M>> for BodyBuilder<'a, E, M> {
    fn build(self) -> ViewBody<'a, E, M> {
        ViewBody::Children(self.children.into_bump_slice())
    }
}

impl<'a, E, M> Builder<ViewBody<'a, E, M>> for &'a str {
    fn build(self) -> ViewBody<'a, E, M> {
        ViewBody::Text(self, self.width() as u16)
    }
}

pub trait Builder<T> {
    fn build(self) -> T;
}

impl<'a> Builder<&'a [&'a str]> for ClassBuilder<'a> {
    fn build(self) -> &'a [&'a str] {
        self.classes.into_bump_slice()
    }
}

impl<'a, E, M> Builder<&'a [EventFilter<'a, E, M>]> for EventBuilder<'a, E, M> {
    fn build(self) -> &'a [EventFilter<'a, E, M>] {
        self.events.into_bump_slice()
    }
}

impl<'a, T> Builder<&'a [T]> for () {
    fn build(self) -> &'a [T] {
        &[]
    }
}

pub fn class(b: &Bump) -> ClassBuilder {
    ClassBuilder::new(b)
}

pub fn event<E, M>(b: &Bump) -> EventBuilder<E, M> {
    EventBuilder::new(b)
}

pub fn body<E, M>(b: &Bump) -> BodyBuilder<E, M> {
    BodyBuilder::new(b)
}

pub fn div<'a, E, M>(
    classes: impl Builder<&'a [&'a str]>,
    events: impl Builder<&'a [EventFilter<'a, E, M>]>,
    body: impl Builder<ViewBody<'a, E, M>>,
) -> View<'a, E, M>
where
    E: Clone,
    M: Clone,
{
    View::new(Tag::Div, classes.build(), events.build(), body.build())
}
