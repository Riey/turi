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
use nohash_hasher::IntMap;

use ansi_term::Style;
use simplecss::{
    AttributeOperator as AttrOp,
    Element,
    PseudoClass,
};

#[derive(Default, Clone, Copy)]
pub struct LayoutResult {
    size:         Vec2,
    border:       Rect,
    #[allow(dead_code)]
    padding:      Rect,
    content:      Rect,
    style:        Style,
    border_style: Style,
}

#[derive(Debug)]
pub struct ElementView<'a, E, M> {
    parent: Option<&'a Self>,
    pos:    usize,
    view:   View<'a, E, M>,
}

impl<'a, E, M> ElementView<'a, E, M> {
    pub fn with_view(view: View<'a, E, M>) -> Self {
        Self::with_parent(None, 0, view)
    }

    fn with_parent(
        parent: Option<&'a Self>,
        pos: usize,
        view: View<'a, E, M>,
    ) -> Self {
        Self { parent, pos, view }
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
        let layout = layout_cache[&self.view.hash_tag()];

        printer.with_style(layout.style, |printer| {
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
                    printer.with_style(layout.border_style, |printer| {
                        printer.print_rect();
                    });
                });
            }

            // TODO: padding
        });

        layout
    }

    pub fn layout(
        &self,
        css: &StyleSheet,
        property: CalcCssProperty,
        layout_cache: &mut IntMap<u64, LayoutResult>,
        max_bound: Rect,
    ) -> LayoutResult {
        if !layout_cache.contains_key(&self.view.hash_tag()) {
            let max_size = max_bound.size();
            let (margin_start, margin_size) = property.margin.calc_rect(max_size);
            let border_size = if property.border_width.is_zero() {
                Vec2::new(0, 0)
            } else {
                Vec2::new(1, 1)
            };
            let (padding_start, padding_size) = property.padding.calc_rect(max_size);
            let max_content_bound = max_bound
                .add_start(margin_start + border_size + padding_start)
                .sub_size(margin_size + border_size + padding_size);

            let content_size = match self.view.body() {
                // known size
                ViewBody::Text(_, width) => Vec2::new(width, 1).min(max_content_bound.size()),
                ViewBody::Children(children) => {
                    let mut ret = Vec2::new(0, 0);
                    for (i, child) in children.iter().enumerate() {
                        let child = ElementView::with_parent(Some(self), i, *child);
                        let child_property = css.calc_prop(&child).calc(property);
                        let child_layout = child.layout(
                            css,
                            child_property,
                            layout_cache,
                            max_content_bound.add_start(ret),
                        );
                        ret = ret.max_x(child_layout.size.x).add_y(child_layout.size.y);
                    }

                    ret
                }
            };

            let content_bound = Rect::new(max_content_bound.start(), content_size);
            let padding_bound = Rect::new(
                content_bound.start() - padding_start,
                content_bound.size() + padding_size,
            );
            let border_bound = Rect::new(
                padding_bound.start() - border_size,
                padding_bound.size() + border_size,
            );
            let size = border_bound.size() + margin_start + margin_size;

            layout_cache.insert(self.view.hash_tag(), LayoutResult {
                size,
                border: border_bound,
                padding: padding_bound,
                content: content_bound,
                style: property.style,
                border_style: Style {
                    foreground: property.border_color,
                    ..Default::default()
                },
            });
        }

        layout_cache.get(&self.view.hash_tag()).copied().unwrap()
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
