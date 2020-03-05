use crate::css::{
    AnsiColor,
    CssFontStyle,
    CssProperty,
    CssVal,
};
use enumset::EnumSet;
use simplecss::{
    Declaration,
    Rule,
    Selector,
};

pub struct CssRule<'a> {
    pub selector: Selector<'a>,
    pub property: CssProperty,
}

impl<'a> CssRule<'a> {
    pub fn new(rule: Rule<'a>) -> Self {
        Self {
            selector: rule.selector,
            property: convert_declar(rule.declarations),
        }
    }
}

fn convert_color(css_color: &str) -> Option<AnsiColor> {
    match css_color {
        "transparent" => return None,
        "red" => return Some(AnsiColor::Red),
        "green" => return Some(AnsiColor::Green),
        "blue" => return Some(AnsiColor::Blue),
        "black" => return Some(AnsiColor::Black),
        "white" => return Some(AnsiColor::White),
        "purple" => return Some(AnsiColor::Purple),
        "yellow" => return Some(AnsiColor::Yellow),
        "cyan" => return Some(AnsiColor::Cyan),
        _ => {}
    }

    let color: css_color_parser::Color = match css_color.parse() {
        Ok(color) => color,
        Err(err) => {
            log::error!("Color parsing error: {:?}", err);
            return None;
        }
    };

    Some(AnsiColor::RGB(color.r, color.g, color.b))
}

fn convert_declar<'a>(declarations: Vec<Declaration<'a>>) -> CssProperty {
    let mut property = CssProperty::default();

    for Declaration { name, value, .. } in declarations {
        match name {
            "color" => {
                if value != "inherit" {
                    property.foreground = CssVal::Val(convert_color(value));
                }
            }
            "background" => {
                if value != "inherit" {
                    property.background = CssVal::Val(convert_color(value));
                }
            }
            "font" => {
                if value.contains("italic") {
                    property
                        .font_style
                        .get_or_insert(EnumSet::new())
                        .insert(CssFontStyle::Italic);
                }
                if value.contains("bold") {
                    property
                        .font_style
                        .get_or_insert(EnumSet::new())
                        .insert(CssFontStyle::Bold);
                }
                if value.contains("hidden") {
                    property
                        .font_style
                        .get_or_insert(EnumSet::new())
                        .insert(CssFontStyle::Hidden);
                }
                if value.contains("reverse") {
                    property
                        .font_style
                        .get_or_insert(EnumSet::new())
                        .insert(CssFontStyle::Reverse);
                }
                if value.contains("dimmed") {
                    property
                        .font_style
                        .get_or_insert(EnumSet::new())
                        .insert(CssFontStyle::Dimmed);
                }
            }
            "text-decoration-line" => {
                if value.contains("blink") {
                    property
                        .font_style
                        .get_or_insert(EnumSet::new())
                        .insert(CssFontStyle::Blink);
                }
                if value.contains("underline") {
                    property
                        .font_style
                        .get_or_insert(EnumSet::new())
                        .insert(CssFontStyle::Underline);
                }
                if value.contains("line-through") {
                    property
                        .font_style
                        .get_or_insert(EnumSet::new())
                        .insert(CssFontStyle::StrikeThrough);
                }
            }
            _ => {}
        }
    }
    property
}
