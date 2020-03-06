use crate::{
    css::{
        Combine,
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
        view: &ElementView<'a, E, M>,
    ) -> CssProperty {
        let mut prop = CssProperty::default();

        for rule in self.rules.iter() {
            if rule.selector.matches(view) {
                prop = prop.combine(rule.property);
            }
        }

        prop
    }
}

#[test]
fn style_test() {
    use crate::{
        builder::{
            class,
            div,
        },
        css::{
            Calc,
            CssSize,
            CssVal,
            StyleSheet,
        },
    };
    use bumpalo::Bump;
    let css = StyleSheet::parse("div { height: 10; } div.hello { width: 10; }");

    let b = Bump::new();
    let view = div(class(&b).class("hello"), (), "Hi");
    let element = ElementView::<(), ()>::with_view(view);

    let prop = css.calc_prop(&element);
    assert_eq!(prop.width, CssVal::Val(CssSize::Fixed(10)));
    assert_eq!(prop.height, CssVal::Val(CssSize::Fixed(10)));

    let calc_prop = prop.calc(Default::default());
    assert_eq!(calc_prop.width, CssSize::Fixed(10));
    assert_eq!(calc_prop.height, CssSize::Fixed(10));
}
