use crate::view::{
    Tag,
    View,
    ViewState,
};
pub use ansi_term::{
    Color,
    Style,
};

use simplecss::{
    AttributeOperator as AttrOp,
    Element,
    PseudoClass,
    Rule as SRule,
    StyleSheet as SStyleSheet,
};

impl<'a, 'p, E, M> Clone for ElementView<'a, 'p, E, M> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, 'p, E, M> Copy for ElementView<'a, 'p, E, M> {}

struct ElementView<'a, 'p, E, M> {
    parent: Option<&'p Self>,
    pos:    usize,
    view:   View<'a, E, M>,
}

impl<'a, 'p, E, M> ElementView<'a, 'p, E, M> {
    pub fn with_view(view: View<'a, E, M>) -> ElementView<'a, 'static, E, M> {
        ElementView {
            parent: None,
            pos: 0,
            view,
        }
    }

    pub fn with_parent(
        parent: &'p Self,
        pos: usize,
    ) -> Option<Self> {
        Some(Self {
            parent: Some(parent),
            pos,
            view: parent.view.children().get(pos).copied()?,
        })
    }
}

impl<'a, 'p, E, M> Element for ElementView<'a, 'p, E, M> {
    fn parent_element(&self) -> Option<Self> {
        self.parent.copied()
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        Self::with_parent(self.parent?, self.pos.checked_sub(1)?)
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
        let attr = self.view.attr();
        match local_name {
            "class" => {
                match op {
                    AttrOp::Contains(name) => attr.class.iter().any(|class| class.contains(name)),
                    AttrOp::Matches(name) => attr.class.iter().any(|class| *class == name),
                    AttrOp::Exists => !attr.class.is_empty(),
                    AttrOp::StartsWith(name) => {
                        attr.class.iter().any(|class| class.starts_with(name))
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

pub use simplecss::Selector;

pub struct Rule<'a> {
    pub selector: Selector<'a>,
    pub style:    Style,
}

pub struct StyleSheet<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> StyleSheet<'a> {
    pub fn calc_style<E, M>(
        &self,
        view: View<E, M>,
    ) -> Style {
        Style::new()
    }
}
