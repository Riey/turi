use crate::view::{View, Tag};
pub use ansi_term::{
    Color,
    Style,
};

use simplecss::Element;
use simplecss::Rule as SRule;
use simplecss::StyleSheet as SStyleSheet;

impl<'a, 'p, E, M> Clone for ElementView<'a, 'p, E, M> {
    fn clone(&self) -> Self { *self }
}
impl<'a, 'p, E, M> Copy for ElementView<'a, 'p, E, M> {
}

struct ElementView<'a, 'p, E, M> {
    parent: &'p Option<Self>,
    view: View<'a, E, M>,
}

impl<'a, 'p, E, M> Element for ElementView<'a, 'p, E, M> {
    fn parent_element(&self) -> Option<Self> { *self.parent }
    fn prev_sibling_element(&self) -> Option<Self> { unimplemented!() }
    fn has_local_name(&self, name: &str) -> bool { unimplemented!() }
    fn attribute_matches(&self, local_name: &str, operator: simplecss::AttributeOperator) -> bool { unimplemented!() }
    fn pseudo_class_matches(&self, class: simplecss::PseudoClass) -> bool { unimplemented!() }
    
}

pub use simplecss::Selector;

pub struct Rule<'a> {
    pub selector: Selector<'a>,
    pub style: Style,
}

pub struct StyleSheet<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> StyleSheet<'a> {
    pub fn calc_style<E, M>(&self, view: View<E, M>) -> Style {
        Style::new()
    }
}


