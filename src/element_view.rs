use crate::{
    css::{
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
use nohash_hasher::IntMap;

use ansi_term::Style;
use simplecss::{
    AttributeOperator as AttrOp,
    Element,
    PseudoClass,
};

#[derive(Default, Clone, Copy)]
pub struct LayoutResult {
    size:     Vec2,
    border:   Rect,
    padding:  Rect,
    content:  Rect,
    property: CalcCssProperty,
}

#[derive(Debug)]
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
            view: self.view.children().get(pos).copied()?,
        })
    }

    pub fn view(self) -> View<'a, E, M> {
        self.view
    }

    pub fn render(
        self,
        css: &StyleSheet,
        printer: &mut Printer,
        layout_cache: &mut IntMap<u64, LayoutResult>,
    ) -> LayoutResult {
        let layout = *layout_cache
            .entry(self.view.hash_tag())
            .or_insert_with(|| self.layout(css, printer.bound()));

        printer.with_style(layout.property.style, |printer| {
            // content
            printer.with_bound(layout.content, |printer| {
                match self.view.body() {
                    ViewBody::Text(text, _) => {
                        printer.print((0, 0), text);
                    }
                    ViewBody::Children(children) => {
                        let mut bound = printer.bound();

                        for pos in 0..children.len() {
                            let child = self.make_child(pos).unwrap();
                            printer.with_bound(bound, |printer| {
                                let layout = child.render(css, printer, layout_cache);
                                bound = bound.add_start((0, layout.size.y));
                            });
                        }
                    }
                }
            });

            // border
            if layout.border.x() > 0 {
                printer.with_bound(layout.border, |printer| {
                    let mut style = Style::default();
                    style.foreground = layout.property.border_color;
                    printer.with_style(style, |printer| {
                        printer.print_rect();
                    });
                });
            }

            // TODO: padding
        });

        layout
    }

    pub fn layout(
        self,
        css: &StyleSheet,
        max_bound: Rect,
    ) -> LayoutResult {
        todo!()
        /*
        self.property = css.calc_prop(self).calc(self.parent.map(|p| p.property).unwrap_or_default());
        let mut bound = max_bound;
        bound = self.property.margin.calc_bound(bound);

        if !self.property.border_width.is_zero() {
            self.layout.border = bound;
            bound = bound.with_margin(1);
        }

        self.layout.padding = bound;
        bound = self.property.padding.calc_bound(bound);

        let content_bound = match self.view.body() {
            ViewBody::Text(_, width) => {
                Rect::new(bound.start(), (width, 1))
            }
            ViewBody::Children(children) => {

            }
        }

        bound
        */
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
