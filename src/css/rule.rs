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
