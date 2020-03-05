use crate::{
    event_filter::EventFilter,
    view::{
        Tag,
        View,
    },
};
use bumpalo::{
    collections::Vec,
    Bump,
};

pub struct ViewBuilder<'a, E, M> {
    tag:        Tag,
    classes:    Vec<'a, &'a str>,
    events:     Vec<'a, EventFilter<'a, E, M>>,
    children:   Vec<'a, View<'a, E, M>>,
    inner_text: &'a str,
}

impl<'a, E, M> ViewBuilder<'a, E, M>
where
    E: Clone,
    M: Clone,
{
    pub fn new(
        b: &'a Bump,
        tag: Tag,
    ) -> Self {
        Self {
            tag,
            classes: Vec::new_in(b),
            events: Vec::new_in(b),
            children: Vec::new_in(b),
            inner_text: "",
        }
    }

    #[inline]
    pub fn class(
        mut self,
        class: &'a str,
    ) -> Self {
        self.classes.push(class);
        self
    }

    #[inline]
    pub fn event(
        mut self,
        event: EventFilter<'a, E, M>,
    ) -> Self {
        self.events.push(event);
        self
    }

    #[inline]
    pub fn child(
        mut self,
        child: View<'a, E, M>,
    ) -> Self {
        self.children.push(child);
        self
    }

    #[inline]
    pub fn children(
        mut self,
        children: impl AsRef<[View<'a, E, M>]>,
    ) -> Self {
        for child in children.as_ref() {
            self.children.push(child.clone());
        }
        self
    }

    #[inline]
    pub fn inner_text(
        mut self,
        inner_text: &'a str,
    ) -> Self {
        self.inner_text = inner_text;
        self
    }

    #[inline]
    pub fn build(self) -> View<'a, E, M> {
        if self.children.is_empty() {
            View::with_inner_text(
                self.tag,
                self.classes.into_bump_slice(),
                self.events.into_bump_slice(),
                self.inner_text,
            )
        } else {
            View::with_children(
                self.tag,
                self.classes.into_bump_slice(),
                self.events.into_bump_slice(),
                self.children.into_bump_slice_mut(),
            )
        }
    }
}

pub fn div<E, M>(b: &Bump) -> ViewBuilder<E, M>
where
    E: Clone,
    M: Clone,
{
    ViewBuilder::new(b, Tag::Div)
}

pub fn text<'a, E, M>(text: &'a str) -> View<'a, E, M> {
    View::with_inner_text(Tag::Div, &[], &[], text)
}
