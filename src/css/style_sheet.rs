use crate::{
    css::{
        Combine,
        CssProperty,
        CssRule,
    },
    element_view::ElementView,
};
use simplecss::StyleSheet as SStyleSheet;

#[derive(Debug)]
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
            body,
            class,
            div,
        },
        css::{
            AnsiColor,
            Calc,
            CssSize,
            CssVal,
            StyleSheet,
        },
    };
    use bumpalo::Bump;
    let css = StyleSheet::parse(include_str!("../../res/simple.css"));

    let b = Bump::new();
    let view = div(
        (),
        (),
        body(&b)
            .child(div(class(&b).class("hello"), (), "Hi"))
            .child(div((), (), "wi")),
    );
    let element = ElementView::<(), ()>::with_view(view);
    let element_prop = css.calc_prop(&element).calc(Default::default());
    let child = element.make_child(0).unwrap();

    assert_eq!(child.view().classes(), &["hello"]);

    let prop = css.calc_prop(&child);
    assert_eq!(prop.width, CssVal::Val(CssSize::Fixed(10)));
    assert_eq!(prop.height, CssVal::Val(CssSize::Fixed(10)));

    let calc_prop = prop.calc(element_prop);
    assert_eq!(calc_prop.width, CssSize::Fixed(10));
    assert_eq!(calc_prop.height, CssSize::Fixed(10));
    assert_eq!(calc_prop.style.foreground, Some(AnsiColor::Red));
}
