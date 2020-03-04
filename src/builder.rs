use crate::{
    attribute::Attribute,
    event_filter::EventFilter,
    view::{
        Tag,
        View,
        ViewInner,
    },
};
use bumpalo::{
    collections::Vec,
    Bump,
};
use unicode_width::UnicodeWidthStr;

pub struct EventBuilder<'a, E, M> {
    events: Vec<'a, EventFilter<'a, E, M>>,
}

impl<'a, E, M> EventBuilder<'a, E, M> {
    pub fn new(b: &'a Bump) -> Self {
        Self {
            events: Vec::new_in(b),
        }
    }

    pub fn event(
        mut self,
        event: EventFilter<'a, E, M>,
    ) -> Self {
        self.events.push(event);
        self
    }

    pub fn build(self) -> &'a [EventFilter<'a, E, M>] {
        self.events.into_bump_slice()
    }
}

pub struct AttributeBuilder<'a, E, M> {
    class:  Vec<'a, &'a str>,
    events: Vec<'a, EventFilter<'a, E, M>>,
}

impl<'a, E, M> AttributeBuilder<'a, E, M> {
    pub fn new(b: &'a Bump) -> Self {
        Self {
            class:  Vec::new_in(b),
            events: Vec::new_in(b),
        }
    }

    pub fn class(
        mut self,
        class: &'a str,
    ) -> Self {
        self.class.push(class);
        self
    }

    pub fn event(
        mut self,
        event: EventFilter<'a, E, M>,
    ) -> Self {
        self.events.push(event);
        self
    }

    pub fn build(self) -> Attribute<'a, E, M> {
        Attribute::new(self.class.into_bump_slice(), self.events.into_bump_slice())
    }
}

pub struct ViewBuilder<'a, E, M> {
    tag:        Tag,
    attr:       Attribute<'a, E, M>,
    children:   Vec<'a, View<'a, E, M>>,
    inner_text: &'a str,
}

impl<'a, E, M> ViewBuilder<'a, E, M> {
    pub fn new(
        b: &'a Bump,
        tag: Tag,
    ) -> Self {
        Self {
            tag,
            attr: Default::default(),
            children: Vec::new_in(b),
            inner_text: "",
        }
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
        self.children.extend_from_slice(children.as_ref());
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
    pub fn attr(
        mut self,
        attr: Attribute<'a, E, M>,
    ) -> Self {
        self.attr = attr;
        self
    }

    #[inline]
    pub fn build(self) -> View<'a, E, M> {
        if self.children.is_empty() {
            View::new(
                self.tag,
                self.attr,
                ViewInner::Text(self.inner_text, self.inner_text.width() as u16),
            )
        } else {
            View::new(
                self.tag,
                self.attr,
                ViewInner::Children(self.children.into_bump_slice()),
            )
        }
    }
}

pub fn attr<E, M>(b: &Bump) -> AttributeBuilder<E, M> {
    AttributeBuilder::new(b)
}

pub fn div<E, M>(b: &Bump) -> ViewBuilder<E, M> {
    ViewBuilder::new(b, Tag::Div)
}

pub fn text<'a, E, M>(text: &'a str) -> View<'a, E, M> {
    View::new(
        Tag::Div,
        Default::default(),
        ViewInner::Text(text, text.width() as u16),
    )
}
