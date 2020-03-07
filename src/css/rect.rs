use crate::{
    css::{
        calc::Calc,
        Combine,
        CssSize,
        CssVal,
    },
    rect::Rect,
    vec2::Vec2,
};
use core::str::FromStr;

impl Combine for CssRect {
    fn combine(
        self,
        other: Self,
    ) -> Self {
        Self {
            top:    self.top.combine(other.top),
            left:   self.left.combine(other.left),
            right:  self.right.combine(other.right),
            bottom: self.bottom.combine(other.bottom),
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct CalcCssRect {
    pub top:    CssSize,
    pub bottom: CssSize,
    pub left:   CssSize,
    pub right:  CssSize,
}

impl CalcCssRect {
    pub fn calc_rect(
        self,
        size: Vec2,
    ) -> (Vec2, Vec2) {
        (
            Vec2::new(self.left.calc_size(size.x), self.top.calc_size(size.y)),
            Vec2::new(self.right.calc_size(size.x), self.bottom.calc_size(size.y)),
        )
    }

    pub fn calc_bound(
        self,
        bound: Rect,
    ) -> Rect {
        let (start, size) = self.calc_rect(bound.size());
        bound.add_start(start).sub_size(size)
    }
}

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct CssRect {
    pub top:    CssVal<CssSize>,
    pub bottom: CssVal<CssSize>,
    pub left:   CssVal<CssSize>,
    pub right:  CssVal<CssSize>,
}

impl Calc for CssRect {
    type Output = CalcCssRect;

    fn calc(
        self,
        parent: Self::Output,
    ) -> Self::Output {
        CalcCssRect {
            top:    self.top.calc(parent.top),
            bottom: self.bottom.calc(parent.bottom),
            left:   self.left.calc(parent.left),
            right:  self.right.calc(parent.right),
        }
    }
}

impl FromStr for CssRect {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ').map(|s| s.parse());

        let first = parts.next().ok_or(())??;

        match parts.next() {
            Some(second) => {
                let second = second?;
                match parts.next() {
                    Some(third) => {
                        let third = third?;
                        match parts.next() {
                            Some(forth) => {
                                let forth = forth?;
                                Ok(CssRect {
                                    top:    first,
                                    bottom: third,
                                    left:   forth,
                                    right:  second,
                                })
                            }
                            None => {
                                Ok(CssRect {
                                    top:    first,
                                    bottom: third,
                                    left:   second,
                                    right:  second,
                                })
                            }
                        }
                    }
                    None => {
                        Ok(CssRect {
                            top:    first,
                            bottom: first,
                            left:   second,
                            right:  second,
                        })
                    }
                }
            }
            None => {
                Ok(CssRect {
                    top:    first,
                    bottom: first,
                    left:   first,
                    right:  first,
                })
            }
        }
    }
}

#[test]
fn parse_test() {
    assert_eq!(
        Ok(CssRect {
            top:    CssVal::Val(CssSize::Fixed(12)),
            bottom: CssVal::Val(CssSize::Fixed(12)),
            left:   CssVal::Val(CssSize::Fixed(12)),
            right:  CssVal::Val(CssSize::Fixed(12)),
        }),
        "12".parse()
    );
}

#[test]
fn calc_test() {
    let ret = CssVal::Val(CssRect {
        top: CssVal::Val(CssSize::Fixed(12)),
        ..Default::default()
    })
    .nest_calc(CalcCssRect {
        ..Default::default()
    });
    assert_eq!(ret.top, CssSize::Fixed(12));
}
