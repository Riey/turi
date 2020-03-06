use crate::{
    css::{
        Calc,
        CalcCssProperty,
        StyleSheet,
    },
    printer::Printer,
    rect::Rect,
    vec2::Vec2,
    view::{
        View,
        ViewBody,
        ViewState,
    },
};

use ansi_term::Style;
use simplecss::{
    AttributeOperator as AttrOp,
    Element,
    PseudoClass,
};

pub struct ElementView<'a, E, M> {
    parent: Option<&'a Self>,
    pos:    usize,
    view:   View<'a, E, M>,
}

impl<'a, E, M> ElementView<'a, E, M> {
    pub fn with_view(view: View<'a, E, M>) -> Self {
        Self {
            parent: None,
            pos: 0,
            view,
        }
    }

    pub fn make_child(
        &'a self,
        pos: usize,
    ) -> Option<Self> {
        Some(Self {
            parent: Some(self),
            pos,
            view: self.view.children().get(pos).cloned()?,
        })
    }

    pub fn render(
        self,
        css: &StyleSheet,
        property: CalcCssProperty,
        printer: &mut Printer,
    ) -> Vec2 {
        let mut view_size = Vec2::new(0, 0);

        printer.with_style(property.style, |printer| {
            let bound = printer.bound();
            let size = bound.size();
            let margin_start = property.margin.calc_start(size);
            let margin_size = property.margin.calc_size(size);
            let border_start = if property.border_width.is_zero() {
                Vec2::new(0, 0)
            } else {
                Vec2::new(1, 1)
            };
            let border_size = border_start;
            let padding_start = property.padding.calc_start(size);
            let padding_size = property.padding.calc_size(size);
            let content_start = margin_start + border_start + padding_start;
            let mut content_size = Vec2::new(0, 0);

            // content
            printer.with_bound(
                bound
                    .add_start(content_start)
                    .sub_size(margin_size + padding_size + border_size),
                |printer| {
                    match self.view.body() {
                        ViewBody::Text(text, width) => {
                            printer.print((0, 0), text);
                            content_size = Vec2::new(width, 1);
                        }
                        ViewBody::Children(children) => {
                            let mut bound = printer.bound();

                            for pos in 0..children.len() {
                                let child = self.make_child(pos).unwrap();
                                let child_property = css.calc_prop(&self).calc(property);
                                let mut child_size = Vec2::new(0, 0);
                                printer.with_bound(bound, |printer| {
                                    child_size = child.render(css, child_property, printer);
                                });
                                bound = bound.add_start((0, child_size.y));
                                log::trace!("child_size {:?}", child_size);
                                content_size = content_size.add_y(child_size.y).max_x(child_size.x);
                            }
                        }
                    }
                },
            );

            log::trace!("content_start {:?}", content_start);
            log::trace!("content_size {:?}", content_size);
            log::trace!("margin {:?} {:?}", margin_start, margin_size);

            view_size = margin_start
                + margin_size
                + border_start
                + border_size
                + padding_start
                + padding_size
                + content_size;

            // border
            if border_start.x > 0 {
                printer.with_bound(
                    Rect::new(
                        bound.start() + margin_start,
                        border_start + border_size + padding_start + padding_size + content_size,
                    ),
                    |printer| {
                        let mut style = Style::default();
                        style.foreground = property.border_color;
                        printer.with_style(style, |printer| {
                            printer.print_rect();
                        });
                    },
                );
            }
        });

        log::trace!("view_size: {:?}", view_size);

        view_size
    }
}

impl<'a, E, M> Element for ElementView<'a, E, M> {
    fn parent_element(&self) -> Option<Self> {
        self.parent.copied()
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.parent?.make_child(self.pos.checked_sub(1)?)
    }

    fn has_local_name(
        &self,
        name: &str,
    ) -> bool {
        Ok(self.view.tag()) == name.parse()
    }

    fn attribute_matches(
        &self,
        local_name: &str,
        op: AttrOp,
    ) -> bool {
        match local_name {
            "class" => {
                match op {
                    AttrOp::Contains(name) => {
                        self.view.classes().iter().any(|class| class.contains(name))
                    }
                    AttrOp::Matches(name) => self.view.classes().iter().any(|class| *class == name),
                    AttrOp::Exists => !self.view.classes().is_empty(),
                    AttrOp::StartsWith(name) => {
                        self.view
                            .classes()
                            .iter()
                            .any(|class| class.starts_with(name))
                    }
                }
            }
            local_name => {
                log::warn!("Unknown attribute local_name: {}", local_name);
                false
            }
        }
    }

    fn pseudo_class_matches(
        &self,
        class: PseudoClass,
    ) -> bool {
        match class {
            PseudoClass::Focus => self.view.has_state(ViewState::Focus),
            PseudoClass::Hover => self.view.has_state(ViewState::Hover),
            _ => false,
        }
    }
}

impl<'a, E, M> Clone for ElementView<'a, E, M> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, E, M> Copy for ElementView<'a, E, M> {}
