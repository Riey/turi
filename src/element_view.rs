use crate::{
    printer::Printer,
    style::{
        Color,
        CssProperty,
        CssSize,
        StyleSheet,
    },
    vec2::Vec2,
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

macro_rules! prop_getter {
    ($name:ident, $ty:ty, $def:expr) => {
        fn $name(self) -> $ty {
            if let Some(ret) = self.property.$name {
                ret
            } else if let Some(parent) = self.parent {
                parent.$name()
            } else {
                $def
            }
        }
    };
}

pub struct ElementView<'a, E, M> {
    parent:   Option<&'a Self>,
    property: CssProperty,
    pos:      usize,
    view:     View<'a, E, M>,
}

impl<'a, E, M> ElementView<'a, E, M> {
    prop_getter!(width, CssSize, CssSize::Percent(100));

    prop_getter!(height, CssSize, CssSize::Percent(100));

    prop_getter!(padding, CssSize, CssSize::Fixed(0));

    prop_getter!(margin, CssSize, CssSize::Fixed(0));

    prop_getter!(border_width, CssSize, CssSize::Fixed(1));

    prop_getter!(border_color, Color, None);

    pub fn with_view(
        view: View<'a, E, M>,
        property: CssProperty,
    ) -> Self {
        Self {
            parent: None,
            property,
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
            property: self.property,
            pos,
            view: self.view.children().get(pos).cloned()?,
        })
    }

    pub fn parent(self) -> Option<&'a Self> {
        self.parent
    }

    pub fn pos(self) -> usize {
        self.pos
    }

    pub fn set_property(
        &mut self,
        property: CssProperty,
    ) {
        self.property = property;
    }

    pub fn property(self) -> CssProperty {
        self.property
    }

    pub fn view(self) -> View<'a, E, M> {
        self.view
    }

    pub fn render(
        self,
        css: &StyleSheet,
        printer: &mut Printer,
    ) {
        printer.with_style(self.property.to_style(printer.style()), |printer| {
            match self.view.body() {
                ViewBody::Text(text, _) => {
                    printer.print((0, 0), text);
                }
                ViewBody::Children(children) => {
                    let mut bound = printer.bound();

                    for (pos, child) in children.iter().enumerate() {
                        printer.with_bound(bound, |printer| {
                            let mut child = self.make_child(pos).unwrap();
                            let property = css.calc_prop(self.property, &child);
                            child.set_property(property);
                            child.render(css, printer);
                        });
                        bound = bound.add_start((0, child.desired_size().y));
                    }
                }
            }
        });
    }

    fn layout(
        self,
        max_size: Vec2,
    ) -> Vec2 {
        todo!()
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
