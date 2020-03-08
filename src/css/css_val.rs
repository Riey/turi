use crate::css::{
    Calc,
    Combine,
};

use std::str::FromStr;

impl<T: Copy + FromStr> FromStr for CssVal<T> {
    type Err = T::Err;

    fn from_str(text: &str) -> Result<Self, T::Err> {
        if text == "inherit" {
            Ok(CssVal::Inherit)
        } else {
            T::from_str(text).map(CssVal::Val)
        }
    }
}

impl<T: Copy + Combine> Combine for CssVal<T> {
    fn combine(
        self,
        rhs: Self,
    ) -> Self {
        match (self, rhs) {
            (CssVal::Val(l), CssVal::Val(r)) => CssVal::Val(l.combine(r)),
            (CssVal::Val(v), _) | (_, CssVal::Val(v)) => CssVal::Val(v),
            _ => CssVal::Inherit,
        }
    }
}

impl<T: Copy> Calc for CssVal<T> {
    type Output = T;

    fn calc(
        self,
        parent: T,
    ) -> Self::Output {
        match self {
            CssVal::Val(v) => v,
            _ => parent,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssVal<T: Copy> {
    Val(T),
    Inherit,
}

impl<T: Copy> Default for CssVal<T> {
    fn default() -> Self {
        CssVal::Inherit
    }
}

impl<T: Copy> CssVal<T> {
    pub fn nest_calc<U>(
        self,
        parent: U,
    ) -> U
    where
        T: Calc<Output = U>,
    {
        match self {
            CssVal::Val(val) => val.calc(parent),
            CssVal::Inherit => parent,
        }
    }

    pub fn unwrap_val(self) -> T {
        match self {
            CssVal::Val(val) => val,
            CssVal::Inherit => panic!("called `CssVal::unwrap_val` on a `Inherit` value"),
        }
    }

    pub fn and_then(
        self,
        f: impl FnOnce(T) -> Self,
    ) -> Self {
        match self {
            CssVal::Val(val) => f(val),
            CssVal::Inherit => CssVal::Inherit,
        }
    }

    pub fn map<U: Copy>(
        self,
        f: impl FnOnce(T) -> U,
    ) -> CssVal<U> {
        match self {
            CssVal::Val(val) => CssVal::Val(f(val)),
            CssVal::Inherit => CssVal::Inherit,
        }
    }

    pub fn get_or_insert(
        &mut self,
        v: T,
    ) -> &mut T {
        if let CssVal::Inherit = *self {
            *self = CssVal::Val(v);
        }

        match self {
            CssVal::Val(val) => val,
            CssVal::Inherit => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}
