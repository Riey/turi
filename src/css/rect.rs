use crate::css::{
    CssSize,
    CssVal,
};

#[derive(Clone, Copy, Default)]
pub struct CssRect {
    pub top:    CssVal<CssSize>,
    pub left:   CssVal<CssSize>,
    pub right:  CssVal<CssSize>,
    pub bottom: CssVal<CssSize>,
}
