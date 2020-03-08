use crate::css::{
    CssFontStyle,
    CssProperty,
};
use enumset::EnumSet;
use simplecss::{
    Declaration,
    Rule,
    Selector,
};

#[derive(Debug)]
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

fn convert_declar<'a>(declarations: Vec<Declaration<'a>>) -> CssProperty {
    let mut property = CssProperty::default();

    macro_rules! set_style {
        ($style:expr) => {
            property
                .font_style
                .get_or_insert(EnumSet::new())
                .insert($style);
        };
    }

    for Declaration { name, value, .. } in declarations {
        match name {
            "color" => {
                if let Ok(color) = value.parse() {
                    property.foreground = color;
                }
            }
            "background" => {
                if let Ok(color) = value.parse() {
                    property.background = color;
                }
            }
            "margin" => {
                if let Ok(width) = value.parse() {
                    property.margin = width;
                }
            }
            "padding" => {
                if let Ok(width) = value.parse() {
                    property.padding = width;
                }
            }
            "border-width" => {
                if let Ok(width) = value.parse() {
                    property.border_width = width;
                }
            }
            "border-color" => {
                if let Ok(color) = value.parse() {
                    property.border_color = color;
                }
            }
            "width" => {
                if let Ok(width) = value.parse() {
                    property.width = width;
                }
            }
            "height" => {
                if let Ok(width) = value.parse() {
                    property.height = width;
                }
            }
            "font-weight" => {
                if value == "bold" || value == "bolder" {
                    set_style!(CssFontStyle::Bold);
                } else if value == "lighter" {
                    set_style!(CssFontStyle::Dimmed);
                }
            }
            "font-style" => {
                if value == "italic" || value == "oblique" {
                    set_style!(CssFontStyle::Italic);
                }
            }
            "text-decoration-line" => {
                if value.contains("blink") {
                    set_style!(CssFontStyle::Blink);
                } else if value.contains("underline") {
                    set_style!(CssFontStyle::Underline);
                } else if value.contains("line-through") {
                    set_style!(CssFontStyle::StrikeThrough);
                }
            }
            _ => {}
        }
    }
    property
}
