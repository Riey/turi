pub use ansi_term::{
    Color as AnsiColor,
    Style,
};
pub type Color = Option<AnsiColor>;
use enumset::{
    EnumSet,
    EnumSetType,
};

use crate::element_view::ElementView;
use simplecss::{
    Declaration,
    Rule as SRule,
    StyleSheet as SStyleSheet,
};

pub use simplecss::Selector;

#[derive(Clone, Copy)]
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
    pub fn combine(
        self,
        rhs: Self,
    ) -> Self {
        match (self, rhs) {
            (CssVal::Val(v), _) | (_, CssVal::Val(v)) => CssVal::Val(v),
            _ => CssVal::Inherit,
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
pub enum CssSize {
    Fixed(u16),
    Percent(u16),
}

impl CssSize {
    pub fn calc(
        self,
        max: u16,
    ) -> u16 {
        let want = match self {
            CssSize::Fixed(x) => x,
            CssSize::Percent(p) => max * 100 / p,
        };

        want.min(max)
    }
}

#[derive(Clone, Copy, Default)]
pub struct CssRect {
    pub top:    CssVal<CssSize>,
    pub left:   CssVal<CssSize>,
    pub right:  CssVal<CssSize>,
    pub bottom: CssVal<CssSize>,
}

#[derive(Clone, Copy, Default)]
pub struct CssProperty {
    pub foreground:   CssVal<Color>,
    pub background:   CssVal<Color>,
    pub font_style:   CssVal<EnumSet<CssFontStyle>>,
    pub width:        CssVal<CssSize>,
    pub height:       CssVal<CssSize>,
    pub padding:      CssVal<CssRect>,
    pub margin:       CssVal<CssRect>,
    pub border_width: CssVal<CssRect>,
    pub border_color: CssVal<Color>,
}

impl CssProperty {
    pub fn combine(
        self,
        rhs: Self,
    ) -> Self {
        macro_rules! combine {
            ($field:ident) => {
                rhs.$field.combine(self.$field)
            };
        }
        Self {
            foreground:   combine!(foreground),
            background:   combine!(background),
            width:        combine!(width),
            height:       combine!(height),
            padding:      combine!(padding),
            margin:       combine!(margin),
            border_width: combine!(border_width),
            border_color: combine!(border_color),
            font_style:   self
                .font_style
                .and_then(|f| rhs.font_style.map(|rf| f.intersection(rf))),
        }
    }

    pub fn to_style(
        self,
        parent_style: Style,
    ) -> Style {
        let mut ret = parent_style;

        if let CssVal::Val(fg) = self.foreground {
            ret.foreground = fg;
        }

        if let CssVal::Val(bg) = self.background {
            ret.background = bg;
        }

        if let CssVal::Val(font_style) = self.font_style {
            use CssFontStyle::*;

            macro_rules! set_if {
                ($(($flag:expr, $field:ident)$(,)?)+) => {
                    $(
                        if font_style.contains($flag) {
                            ret.$field = true;
                        }
                    )+
                };
            }

            set_if!(
                (Bold, is_bold),
                (Dimmed, is_dimmed),
                (Italic, is_italic),
                (Underline, is_underline),
                (Blink, is_blink),
                (Reverse, is_reverse),
                (Hidden, is_hidden),
                (StrikeThrough, is_strikethrough),
            );
        }

        ret
    }
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

impl<'a, E, M> Clone for ElementView<'a, E, M> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, E, M> Copy for ElementView<'a, E, M> {}
