use super::combine::Combine;
use enumset::{
    EnumSet,
    EnumSetType,
};

impl Combine for EnumSet<CssFontStyle> {
    fn combine(
        self,
        other: Self,
    ) -> Self {
        self.intersection(other)
    }
}

#[derive(EnumSetType)]
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
