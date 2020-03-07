mod auto;
mod calc;
mod color;
mod combine;
mod css_val;
mod font_style;
mod property;
mod rect;
mod rule;
mod size;
mod style_sheet;

use self::{
    combine::Combine,
    rule::CssRule,
};

pub use self::{
    auto::CssAuto,
    calc::Calc,
    color::CssColor,
    css_val::CssVal,
    font_style::CssFontStyle,
    property::{
        CalcCssProperty,
        CssProperty,
    },
    rect::{
        CalcCssRect,
        CssRect,
    },
    size::CssSize,
    style_sheet::StyleSheet,
};

pub use ansi_term::{
    Color as AnsiColor,
    Style as AnsiStyle,
};
pub type Color = Option<AnsiColor>;
