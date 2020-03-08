use super::combine::Combine;
use enumset::{
    EnumSet,
    EnumSetType,
};

impl Combine for EnumSet<CssFontStyle> {
    #[inline]
    fn combine(
        self,
        other: Self,
    ) -> Self {
        self | other
    }
}

#[derive(EnumSetType, Debug)]
pub enum CssFontStyle {
    Bold,
    Dimmed,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    StrikeThrough,
}

#[test]
fn combine_test() {
    let l = CssFontStyle::Bold | CssFontStyle::Dimmed;
    let r = CssFontStyle::Bold | CssFontStyle::Reverse;
    assert_eq!(l.combine(r), l | r);
}
