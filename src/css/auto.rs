use crate::css::{
    Calc,
    Combine,
    CssSize,
};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssAuto<T> {
    Manual(T),
    Auto,
}

impl CssAuto<CssSize> {
    pub fn calc_size(
        self,
        max: u16,
    ) -> Option<u16> {
        match self {
            CssAuto::Manual(val) => Some(val.calc_size(max)),
            _ => None,
        }
    }
}

impl<T> Calc for CssAuto<T> {
    type Output = T;

    fn calc(
        self,
        other: T,
    ) -> T {
        match self {
            CssAuto::Manual(val) => val,
            CssAuto::Auto => other,
        }
    }
}

impl<T> Default for CssAuto<T> {
    fn default() -> Self {
        CssAuto::Auto
    }
}

impl<T> Combine for CssAuto<T> {
    fn combine(
        self,
        other: Self,
    ) -> Self {
        match (self, other) {
            (_, ret @ CssAuto::Manual(_)) | (ret @ CssAuto::Manual(_), _) => ret,
            _ => CssAuto::Auto,
        }
    }
}

impl<T: FromStr> FromStr for CssAuto<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "auto" {
            Ok(CssAuto::Auto)
        } else {
            s.parse().map(CssAuto::Manual)
        }
    }
}
