use crate::css::Color;

pub trait Combine {
    fn combine(
        self,
        other: Self,
    ) -> Self;
}

impl Combine for Color {
    fn combine(
        self,
        other: Self,
    ) -> Self {
        other
    }
}
