use crate::{
    attribute::Attribute,
    event::EventLike,
    printer::Printer,
    vec2::Vec2,
};

use enumset::{
    EnumSet,
    EnumSetType,
};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tag {
    Div,
    Button,
}

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

pub enum ViewInner<'a, E, M> {
    Text(&'a str, u16),
    Children(&'a [View<'a, E, M>]),
}

#[derive(EnumSetType, Debug)]
pub enum ViewState {
    Normal,
    Focus,
    Hover,
}

pub struct View<'a, E, M> {
    tag:   Tag,
    state: EnumSet<ViewState>,
    attr:  Attribute<'a, E, M>,
    inner: ViewInner<'a, E, M>,
}

impl<'a, E, M> View<'a, E, M> {
    pub fn new(
        tag: Tag,
        attr: Attribute<'a, E, M>,
        inner: ViewInner<'a, E, M>,
    ) -> Self {
        Self {
            tag,
            state: EnumSet::new(),
            attr,
            inner,
        }
    }

    #[inline]
    pub fn tag(self) -> Tag {
        self.tag
    }

    #[inline]
    pub fn state(self) -> EnumSet<ViewState> {
        self.state
    }

    #[inline]
    pub fn has_state(self, state: ViewState) -> bool {
        self.state.contains(state)
    }

    pub fn attr(self) -> Attribute<'a, E, M> {
        self.attr
    }

    pub fn children(self) -> &'a [Self] {
        match self.inner {
            ViewInner::Children(children) => children,
            _ => &[],
        }
    }

    pub fn render(
        self,
        printer: &mut Printer,
    ) {
        match &self.inner {
            ViewInner::Text(text, _) => {
                printer.print((0, 0), text);
            }
            ViewInner::Children(children) => {
                let mut bound = printer.bound();

                for child in children.iter() {
                    printer.with_bound(bound, |printer| {
                        child.render(printer);
                    });
                    bound = bound.add_start((0, child.desired_size().y));
                }
            }
        }
    }

    pub fn desired_size(self) -> Vec2 {
        match self.inner {
            ViewInner::Text(_, width) => Vec2::new(width, 1),
            ViewInner::Children(children) => {
                let mut ret = Vec2::new(0, 0);

                for child in children {
                    let size = child.desired_size();
                    ret.x = ret.x.max(size.x);
                    ret.y += size.y;
                }

                ret
            }
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
        for event in self.attr.events {
            if let msg @ Some(_) = event.check(&e) {
                return msg;
            }
        }

        match self.inner {
            ViewInner::Text(..) => None,
            ViewInner::Children(children) => {
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
