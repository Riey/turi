use crate::{
    css::{
        CssProperty,
        CssRule,
    },
    element_view::ElementView,
};
use simplecss::StyleSheet as SStyleSheet;

pub struct StyleSheet<'a> {
    rules: Vec<CssRule<'a>>,
}

impl<'a> StyleSheet<'a> {
    pub fn parse(text: &'a str) -> Self {
        let css = SStyleSheet::parse(text);
        let mut rules: Vec<_> = css.rules.into_iter().map(|r| CssRule::new(r)).collect();
        // reorder rules with selector length
        rules.sort_unstable_by(|l, r| {
            l.selector
                .to_string()
                .len()
                .cmp(&r.selector.to_string().len())
        });
        Self { rules }
    }

    pub fn calc_prop<E, M>(
        &self,
        parent_prop: CssProperty,
        view: &ElementView<'a, E, M>,
    ) -> CssProperty {
        let mut prop = parent_prop;

        for rule in self.rules.iter() {
            if rule.selector.matches(view) {
                prop = prop.combine(rule.property);
            }
        }

        prop
    }
}
