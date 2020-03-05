#[derive(Clone, Copy)]
pub enum CssSize {
    Fixed(u16),
    Percent(u16),
}

impl CssSize {
    pub fn calc(
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
