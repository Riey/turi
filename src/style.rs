pub use ansi_term::{
    Color as AnsiColor,
    Style,
};
pub type Color = Option<AnsiColor>;
use css_color_parser::Color as CssColor;
use enumset::{
    EnumSet,
    EnumSetType,
};

use simplecss::{
    Declaration,
    Rule as SRule,
    StyleSheet as SStyleSheet,
};
use crate::element_view::ElementView;

pub use simplecss::Selector;

#[derive(EnumSetType)]
pub enum CssFontStyle {
    Bold,
    Dimmed,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    StrikeThrough,
}

#[derive(Clone, Copy)]
pub struct CssBorder {
    width: Option<CssSize>,
    // TODO: style
    color: Option<Color>,
}

#[derive(Clone, Copy, Default)]
pub struct CssProperty {
    foreground: Option<Color>,
    background: Option<Color>,
    font_style: Option<EnumSet<CssFontStyle>>,
    width:      Option<CssSize>,
    height:     Option<CssSize>,
    padding:    Option<CssSize>,
    border:     Option<CssBorder>,
}

impl CssProperty {
    pub fn combine(
        self,
        rhs: Self,
    ) -> Self {
        macro_rules! combine {
            ($field:ident) => {
                rhs.$field.or(self.$field)
            };
        }
        Self {
            foreground: combine!(foreground),
            background: combine!(background),
            width:      combine!(width),
            height:     combine!(height),
            padding:    combine!(padding),
            border:     combine!(border),
            font_style: self
                .font_style
                .and_then(|f| rhs.font_style.map(|rf| f.intersection(rf))),
        }
    }

    pub fn to_style(
        self,
        parent_style: Style,
    ) -> Style {
        let mut ret = parent_style;

        if let Some(fg) = self.foreground {
            ret.foreground = fg;
        }

        if let Some(bg) = self.background {
            ret.background = bg;
        }

        if let Some(font_style) = self.font_style {
            use CssFontStyle::*;

            macro_rules! set_if {
                ($(($flag:expr, $field:ident))+) => {
                    $(
                        if font_style.contains($flag) {
                            ret.$field = true;
                        }
                    )+
                };
            }

            set_if!((Bold, is_bold)(Dimmed, is_dimmed)(Italic, is_italic)(
                Underline,
                is_underline
            )(Blink, is_blink)(Reverse, is_reverse)(
                Hidden, is_hidden
            )(StrikeThrough, is_strikethrough));
        }

        ret
    }
}

#[derive(Clone, Copy)]
pub enum CssSize {
    Fixed(u16),
    Percent(u16),
}

#[derive(Clone, Copy)]
pub struct Layout {
    width: Option<CssSize>,
}

pub struct Rule<'a> {
    selector: Selector<'a>,
    property: CssProperty,
}

impl<'a> Rule<'a> {
    pub fn new(rule: SRule<'a>) -> Self {
        Self {
            selector: rule.selector,
            property: convert_declar(rule.declarations),
        }
    }
}

pub struct StyleSheet<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> StyleSheet<'a> {
    pub fn parse(text: &'a str) -> Self {
        let css = SStyleSheet::parse(text);
        let mut rules: Vec<_> = css.rules.into_iter().map(|r| Rule::new(r)).collect();
        // reorder rules with selector length
        rules.sort_unstable_by(|l, r| {
            l.selector
                .to_string()
                .len()
                .cmp(&r.selector.to_string().len())
        });
        Self { rules }
    }

    pub fn calc_prop<E, M>(
        &self,
        parent_prop: CssProperty,
        view: &ElementView<'a, E, M>,
    ) -> CssProperty {
        let mut prop = parent_prop;

        for rule in self.rules.iter() {
            if rule.selector.matches(view) {
                prop = prop.combine(rule.property);
            }
        }

        prop
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

    let color: CssColor = match css_color.parse() {
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
                    property.foreground = Some(convert_color(value));
                }
            }
            "background" => {
                if value != "inherit" {
                    property.background = Some(convert_color(value));
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

impl<'a, E, M> Clone for ElementView<'a, E, M> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, E, M> Copy for ElementView<'a, E, M> {}
