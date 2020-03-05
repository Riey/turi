use crate::{
    event::EventLike,
    event_filter::EventFilter,
    printer::Printer,
    style::{
        CssProperty,
        ElementView,
        StyleSheet,
    },
    vec2::Vec2,
};

use enumset::{
    EnumSet,
    EnumSetType,
};
use unicode_width::UnicodeWidthStr;

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

pub struct View<'a, E, M> {
    tag:     Tag,
    classes: &'a [&'a str],
    state:   EnumSet<ViewState>,
    events:  &'a [EventFilter<'a, E, M>],
    body:    ViewBody<'a, E, M>,
}

impl<'a, E, M> View<'a, E, M> {
    pub fn with_children(
        tag: Tag,
        classes: &'a [&'a str],
        events: &'a [EventFilter<'a, E, M>],
        children: &'a [View<'a, E, M>],
    ) -> Self {
        Self {
            tag,
            classes,
            state: EnumSet::new(),
            events,
            body: ViewBody::Children(children),
        }
    }

    pub fn with_inner_text(
        tag: Tag,
        classes: &'a [&'a str],
        events: &'a [EventFilter<'a, E, M>],
        inner_text: &'a str,
    ) -> Self {
        Self {
            tag,
            classes,
            state: EnumSet::new(),
            events,
            body: ViewBody::Text(inner_text, inner_text.width() as u16),
        }
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

    pub fn children(self) -> &'a [Self] {
        match self.body {
            ViewBody::Children(children) => children,
            _ => &[],
        }
    }

    pub fn render(
        self,
        css: &StyleSheet,
        printer: &mut Printer,
    ) where
        E: 'static,
        M: 'static,
    {
        let mut view = ElementView::with_view(self, Default::default());
        view.set_property(css.calc_prop(view.property(), &view));
        Self::render_impl(&view,  css, printer);
    }

    fn render_impl<'e>(
        view: &'e ElementView<'e, E, M>,
        css: &StyleSheet,
        printer: &mut Printer,
    ) where
        E: 'static,
        M: 'static,
    {
        printer.with_style(view.property().to_style(printer.style()), |printer| {
            match view.view().body {
                ViewBody::Text(text, _) => {
                    printer.print((0, 0), text);
                }
                ViewBody::Children(children) => {
                    let mut bound = printer.bound();

                    for (pos, child) in children.iter().enumerate() {
                        printer.with_bound(bound, |printer| {
                            let mut child = view.make_child(pos).unwrap();
                            let property = css.calc_prop(view.property(), &child);
                            child.set_property(property);
                            Self::render_impl(&child, css, printer);
                        });
                        bound = bound.add_start((0, child.desired_size().y));
                    }
                }
            }
        });
    }

    pub fn desired_size(self) -> Vec2 {
        match self.body {
            ViewBody::Text(_, width) => Vec2::new(width, 1),
            ViewBody::Children(children) => {
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

impl<'a, E, M> Clone for ViewBody<'a, E, M> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, E, M> Copy for ViewBody<'a, E, M> {}

impl<'a, E, M> Clone for View<'a, E, M> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, E, M> Copy for View<'a, E, M> {}
