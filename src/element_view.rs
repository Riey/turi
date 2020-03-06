use crate::{
    css::{
        Calc,
        CalcCssProperty,
        StyleSheet,
    },
    printer::Printer,
    view::{
        View,
        ViewBody,
        ViewState,
    },
};

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
        parent_property: CalcCssProperty,
        printer: &mut Printer,
    ) {
        let property = css.calc_prop(&self).calc(parent_property);
        printer.with_style(property.style, |printer| {
            // margin
            printer.with_bound(property.margin.calc_bound(printer.bound()), |printer| {
                let mut bound = printer.bound();
                if !property.border_width.is_zero() {
                    printer.print_rect();
                    bound = bound.add_start((1, 1)).sub_size((1, 1));
                }

                // border
                printer.with_bound(bound, |printer| {
                    // TODO: fill background

                    // padding
                    printer.with_bound(property.padding.calc_bound(printer.bound()), |printer| {
                        // content
                        match self.view.body() {
                            ViewBody::Text(text, _) => {
                                printer.print((0, 0), text);
                            }
                            ViewBody::Children(children) => {
                                let mut bound = printer.bound();

                                for (pos, child) in children.iter().enumerate() {
                                    printer.with_bound(bound, |printer| {
                                        let child = self.make_child(pos).unwrap();
                                        child.render(css, property, printer);
                                    });
                                    bound = bound.add_start((0, child.desired_size().y));
                                }
                            }
                        }
                    })
                });
            });
        })
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
