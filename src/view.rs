use crate::{
    event::{
        EventLike,
        KeyEventLike,
        MouseEventLike,
    },
    printer::Printer,
    vec2::Vec2,
};
use unicode_width::UnicodeWidthStr;

impl<'a, M> Clone for View<'a, M> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, M> Copy for View<'a, M> {}

pub enum View<'a, M> {
    Div(&'a [View<'a, M>]),
    Text(&'a str),
    Button(&'a str, &'a dyn Fn() -> M),
}

impl<'a, M> View<'a, M> {
    pub fn name(self) -> &'static str {
        match self {
            View::Div(..) => "div",
            View::Text(..) => "text",
            View::Button(..) => "button",
        }
    }

    pub fn render(
        self,
        printer: &mut Printer,
    ) {
        match self {
            View::Div(children) => {
                let mut bound = printer.bound();

                for child in children {
                    printer.with_bound(bound, |printer| {
                        child.render(printer);
                    });

                    let child_size = child.desired_size();
                    bound = bound.add_start((0, child_size.y));
                }
            }
            View::Button(text, _) | View::Text(text) => {
                printer.print((0, 0), text);
            }
        }
    }

    pub fn desired_size(self) -> Vec2 {
        match self {
            View::Div(children) => {
                let mut ret = Vec2::new(0, 0);

                for child in children {
                    let size = child.desired_size();
                    ret.x = ret.x.max(size.x);
                    ret.y += size.y;
                }

                ret
            }
            View::Button(text, _) | View::Text(text) => Vec2::new(text.width() as u16, 1),
        }
    }

    pub fn on_event<E: EventLike>(
        self,
        e: E,
    ) -> Option<M> {
        None
    }
}
