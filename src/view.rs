use crate::{
    event::EventLike,
    event_filter::EventFilter,
};

use enumset::{
    EnumSet,
    EnumSetType,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Tag {
    Div,
    Button,
}

#[derive(Debug)]
pub enum ViewBody<'a, E, M> {
    Text(&'a str, u16),
    Children(&'a [View<'a, E, M>]),
}

#[derive(EnumSetType, Debug)]
pub enum ViewState {
    Normal,
    Focus,
    Hover,
}

#[derive(Debug)]
pub struct View<'a, E, M> {
    tag:      Tag,
    state:    EnumSet<ViewState>,
    classes:  &'a [&'a str],
    events:   &'a [EventFilter<'a, E, M>],
    body:     ViewBody<'a, E, M>,
    hash_tag: u64,
}

impl<'a, E, M> View<'a, E, M> {
    pub fn new(
        tag: Tag,
        classes: &'a [&'a str],
        events: &'a [EventFilter<'a, E, M>],
        body: ViewBody<'a, E, M>,
    ) -> Self {
        let mut view = Self {
            tag,
            state: EnumSet::new(),
            classes,
            events,
            body,
            hash_tag: 0,
        };

        use std::hash::{
            Hash,
            Hasher,
        };
        let mut hasher = ahash::AHasher::default();
        view.hash(&mut hasher);
        view.hash_tag = hasher.finish();

        view
    }

    #[inline]
    pub fn hash_tag(self) -> u64 {
        self.hash_tag
    }

    #[inline]
    pub fn tag(self) -> Tag {
        self.tag
    }

    #[inline]
    pub fn classes(self) -> &'a [&'a str] {
        self.classes
    }

    #[inline]
    pub fn state(self) -> EnumSet<ViewState> {
        self.state
    }

    #[inline]
    pub fn has_state(
        self,
        state: ViewState,
    ) -> bool {
        self.state.contains(state)
    }

    #[inline]
    pub fn body(self) -> ViewBody<'a, E, M> {
        self.body
    }

    pub fn children(self) -> &'a [Self] {
        match self.body {
            ViewBody::Children(children) => children,
            _ => &[],
        }
    }
}

impl<'a, E, M> View<'a, E, M>
where
    E: EventLike + Copy,
    M: Copy,
{
    pub fn on_event(
        self,
        e: E,
    ) -> Option<M> {
        for event in self.events {
            if let msg @ Some(_) = event.check(&e) {
                return msg;
            }
        }

        match self.body {
            ViewBody::Text(..) => None,
            ViewBody::Children(children) => {
                for child in children {
                    if let msg @ Some(_) = child.on_event(e) {
                        return msg;
                    }
                }
                None
            }
        }
    }
}

#[doc(hidden)]
mod _impl {
    use super::*;
    use std::hash::{
        Hash,
        Hasher,
    };

    impl std::str::FromStr for Tag {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "div" => Ok(Tag::Div),
                "button" => Ok(Tag::Button),
                _ => Err(()),
            }
        }
    }

    impl<'a, E, M> Clone for ViewBody<'a, E, M> {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl<'a, E, M> Copy for ViewBody<'a, E, M> {}

    impl<'a, E, M> Hash for ViewBody<'a, E, M> {
        fn hash<H: Hasher>(
            &self,
            state: &mut H,
        ) {
            match self {
                ViewBody::Text(text, _) => {
                    text.hash(state);
                }
                ViewBody::Children(children) => {
                    children.hash(state);
                }
            }
        }
    }

    impl<'a, E, M> Eq for ViewBody<'a, E, M> {}

    impl<'a, E, M> PartialEq for ViewBody<'a, E, M> {
        fn eq(
            &self,
            other: &Self,
        ) -> bool {
            match (self, other) {
                (ViewBody::Text(l, _), ViewBody::Text(r, _)) => l == r,
                (ViewBody::Children(l), ViewBody::Children(r)) => l == r,
                _ => false,
            }
        }
    }
    impl<'a, E, M> Clone for View<'a, E, M> {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl<'a, E, M> Copy for View<'a, E, M> {}
    impl<'a, E, M> Hash for View<'a, E, M> {
        fn hash<H: Hasher>(
            &self,
            state: &mut H,
        ) {
            self.tag.hash(state);
            self.classes.hash(state);
            self.state.hash(state);
            self.body.hash(state);
        }
    }

    impl<'a, E, M> Eq for View<'a, E, M> {}

    impl<'a, E, M> PartialEq for View<'a, E, M> {
        fn eq(
            &self,
            other: &Self,
        ) -> bool {
            self.tag == other.tag
                && self.classes == other.classes
                && self.state == other.state
                && self.body == other.body
        }
    }
}
