use crate::{
    attribute::Attribute,
    event::EventLike,
    printer::Printer,
    vec2::Vec2,
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

#[derive(Clone, Copy, Debug)]
pub enum Tag {
    Div,
    Button,
}

pub enum ViewInner<'a, E, M> {
    Text(&'a str, u16),
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
