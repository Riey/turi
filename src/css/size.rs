use crate::css::Combine;
use core::str::FromStr;

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

impl FromStr for CssSize {
    type Err = ();

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('%') {
            s.split_at(s.len() - 1)
                .0
                .parse()
                .map(CssSize::Percent)
                .map_err(|_| ())
        } else {
            s = s.trim_end_matches("px");
            s.parse().map(CssSize::Fixed).map_err(|_| ())
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[test]
fn parse_test() {
    assert_eq!(Ok(CssSize::Fixed(123)), "123".parse());
    assert_eq!(Ok(CssSize::Fixed(4)), "4px".parse());
}
