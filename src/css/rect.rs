use crate::{
    css::{
        calc::Calc,
        Combine,
        CssSize,
        CssVal,
    },
    rect::Rect,
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

#[derive(Clone, Copy, Default)]
pub struct CalcCssRect {
    pub top:    CssSize,
    pub bottom: CssSize,
    pub left:   CssSize,
    pub right:  CssSize,
}

impl CalcCssRect {
    pub fn calc_bound(
        self,
        bound: Rect,
    ) -> Rect {
        let w = bound.w();
        let h = bound.h();
        bound
            .add_start((self.left.calc_size(w), self.top.calc_size(h)))
            .sub_size((self.right.calc_size(w), self.bottom.calc_size(h)))
    }
}

#[derive(Clone, Copy, Default, Debug)]
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
