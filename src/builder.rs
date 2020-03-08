use crate::{
    event::{
        EventLike,
        KeyEventLike,
        MouseEventLike,
    },
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
        self.events.push(EventFilter::new(
            self.b.alloc(move |e: &E| {
                e.try_key()
                    .and_then(|ke| ke.try_ctrl_char())
                    .map_or(false, |c| c == ch)
            }),
            msg,
        ));
        self
    }

    pub fn click(
        mut self,
        msg: M,
    ) -> Self
    where
        E: EventLike,
        M: Copy,
    {
        self.events.push(EventFilter::new(
            &|e| {
                e.try_mouse()
                    .and_then(|me| me.try_left_down())
                    .map_or(false, |_| true)
            },
            msg,
        ));
        self
    }
}

pub struct BodyBuilder<'a, E, M> {
    b:        &'a Bump,
    children: Vec<'a, View<'a, E, M>>,
}

impl<'a, E, M> BodyBuilder<'a, E, M> {
    pub fn new(b: &'a Bump) -> Self {
        Self {
            b,
            children: Vec::with_capacity_in(3, b),
        }
    }

    pub fn text(
        self,
        text: &str,
    ) -> &'a str {
        self.b.alloc_str(text)
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

impl<T> Builder<T> for T {
    fn build(self) -> T {
        self
    }
}

pub fn class_ref<'a>(
    b: &'a Bump,
    classes: impl AsRef<[&'a str]>,
) -> &'a [&'a str] {
    b.alloc_slice_copy(classes.as_ref())
}

pub fn class<'a, 'b>(
    b: &'a Bump,
    classes: impl AsRef<[&'b str]>,
) -> &'a [&'a str] {
    let classes = classes.as_ref();
    b.alloc_slice_fill_iter(classes.iter().map(|class| b.alloc_str(class) as &str)) as &[_]
}

pub fn event<E, M>(b: &Bump) -> EventBuilder<E, M> {
    EventBuilder::new(b)
}

pub fn body<E, M>(b: &Bump) -> BodyBuilder<E, M> {
    BodyBuilder::new(b)
}

/// Build div view
///
/// # Examples
///
/// ```
/// use turi::{
///     builder::{
///         body,
///         class_ref,
///         div,
///         event,
///     },
///     Model,
///     UpdateResult,
///     Exit,
///     Ignore,
///     Bump,
///     View,
/// };
///
/// struct Simple;
///
/// impl Model<()> for Simple {
///     type Msg = bool;
///
///     fn update(
///         &mut self,
///         msg: Self::Msg,
///     ) -> UpdateResult {
///         if msg {
///             Exit
///         } else {
///             Ignore
///         }
///     }
///
///     fn view<'a>(
///         &self,
///         b: &'a Bump,
///     ) -> View<'a, (), Self::Msg> {
///         div(
///             (),
///             event(b).ctrl_char('c', true),
///             body(b)
///                 .child(div(class_ref(b, ["hello"]), (), "Hello"))
///                 .child(div((), (), "World!")),
///         )
///     }
/// }
/// ```
pub fn div<'a, E, M>(
    classes: impl Builder<&'a [&'a str]>,
    events: impl Builder<&'a [EventFilter<'a, E, M>]>,
    body: impl Builder<ViewBody<'a, E, M>>,
) -> View<'a, E, M> {
    View::new(Tag::Div, classes.build(), events.build(), body.build())
}
