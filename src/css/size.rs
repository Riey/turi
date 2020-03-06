use crate::css::Combine;

impl Combine for CssSize {
    fn combine(
        self,
        other: Self,
    ) -> Self {
        match (self, other) {
            (CssSize::Percent(x), CssSize::Percent(y)) => CssSize::Percent(x.max(y)),
            (CssSize::Percent(p), _) | (_, CssSize::Percent(p)) => CssSize::Percent(p),
            (CssSize::Fixed(x), CssSize::Fixed(y)) => CssSize::Fixed(x.max(y)),
        }
    }
}

impl Default for CssSize {
    fn default() -> Self {
        CssSize::Fixed(0)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CssSize {
    Fixed(u16),
    Percent(u16),
}

impl CssSize {
    pub fn is_zero(self) -> bool {
        match self {
            CssSize::Fixed(x) | CssSize::Percent(x) => x == 0,
        }
    }

    pub fn calc_size(
        self,
        max: u16,
    ) -> u16 {
        let want = match self {
            CssSize::Fixed(x) => x,
            CssSize::Percent(p) => max * 100 / p,
        };

        want.min(max)
    }
}
