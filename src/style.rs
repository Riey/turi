use crate::view::{
    View,
    ViewState,
};
pub use ansi_term::{
    Color,
    Style,
};
use css_color_parser::Color as CssColor;

use simplecss::{
    AttributeOperator as AttrOp,
    Declaration,
    Element,
    PseudoClass,
    Rule as SRule,
    StyleSheet as SStyleSheet,
};

pub struct ElementView<'a, E, M> {
    parent: Option<&'a Self>,
    pos:    usize,
    view:   View<'a, E, M>,
}

impl<'a, E, M> ElementView<'a, E, M> {
    pub fn with_view(view: View<'a, E, M>) -> Self {
        Self {
            parent: None,
            pos: 0,
            view,
        }
    }

    pub fn make_child(&'a self, pos: usize) -> Option<Self> {
        Some(Self {
            parent: Some(self),
            pos,
            view: self.view.children().get(pos).cloned()?,
        })
    }
}

impl<'a, E, M> Element for ElementView<'a, E, M> {
    fn parent_element(&self) -> Option<Self> {
        self.parent.copied()
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.parent?.make_child(self.pos.checked_sub(1)?)
    }

    fn has_local_name(
        &self,
        name: &str,
    ) -> bool {
        Ok(self.view.tag()) == name.parse()
    }

    fn attribute_matches(
        &self,
        local_name: &str,
        op: AttrOp,
    ) -> bool {
        match local_name {
            "class" => {
                match op {
                    AttrOp::Contains(name) => {
                        self.view.classes().iter().any(|class| class.contains(name))
                    }
                    AttrOp::Matches(name) => self.view.classes().iter().any(|class| *class == name),
                    AttrOp::Exists => !self.view.classes().is_empty(),
                    AttrOp::StartsWith(name) => {
                        self.view.classes().iter().any(|class| class.starts_with(name))
                    }
                }
            }
            local_name => {
                log::warn!("Unknown attribute local_name: {}", local_name);
                false
            }
        }
    }

    fn pseudo_class_matches(
        &self,
        class: PseudoClass,
    ) -> bool {
        match class {
            PseudoClass::Focus => self.view.has_state(ViewState::Focus),
            PseudoClass::Hover => self.view.has_state(ViewState::Hover),
            _ => false,
        }
    }
}

pub use simplecss::Selector;

#[derive(Clone, Copy)]
struct CssStyle {
    style:                 Style,
    foreground_is_inherit: bool,
    background_is_inherit: bool,
}

impl CssStyle {
    pub fn to_style(
        self,
        parent: Style,
    ) -> Style {
        let mut ret = self.style;
        if self.foreground_is_inherit {
            ret.foreground = parent.foreground;
        }

        if self.background_is_inherit {
            ret.background = parent.background;
        }

        ret
    }
}

pub struct Rule<'a> {
    selector: Selector<'a>,
    style:    CssStyle,
}

impl<'a> Rule<'a> {
    pub fn new(rule: SRule<'a>) -> Self {
        Self {
            selector: rule.selector,
            style:    convert_declar(rule.declarations),
        }
    }
}

pub struct StyleSheet<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> StyleSheet<'a> {
    pub fn parse(text: &'a str) -> Self {
        let css = SStyleSheet::parse(text);
        Self {
            rules: css.rules.into_iter().map(|r| Rule::new(r)).collect(),
        }
    }

    pub fn calc_style<E, M>(
        &self,
        parent_style: Style,
        view: &ElementView<'a, E, M>,
    ) -> Style {
        for rule in self.rules.iter() {
            if rule.selector.matches(view) {
                return rule.style.to_style(parent_style);
            }
        }

        parent_style
    }
}

fn convert_color(css_color: &str) -> Option<Color> {
    match css_color {
        "transparent" => return None,
        "red" => return Some(Color::Red),
        "green" => return Some(Color::Green),
        "blue" => return Some(Color::Blue),
        "black" => return Some(Color::Black),
        "white" => return Some(Color::White),
        "purple" => return Some(Color::Purple),
        "yellow" => return Some(Color::Yellow),
        "cyan" => return Some(Color::Cyan),
        _ => {}
    }

    let color: CssColor = match css_color.parse() {
        Ok(color) => color,
        Err(err) => {
            log::error!("Color parsing error: {:?}", err);
            return None;
        }
    };

    Some(Color::RGB(color.r, color.g, color.b))
}

fn convert_declar<'a>(declarations: Vec<Declaration<'a>>) -> CssStyle {
    let mut ret = Style::new();
    let mut foreground_is_inherit = true;
    let mut background_is_inherit = true;

    for Declaration { name, value, .. } in declarations {
        match name {
            "color" => {
                if value != "inherit" {
                    ret.foreground = convert_color(value);
                    foreground_is_inherit = false;
                }
            }
            "background" => {
                if value != "inherit" {
                    ret.background = convert_color(value);
                    background_is_inherit = false;
                }
            }
            "font" => {
                ret.is_italic = value.contains("italic");
                ret.is_bold = value.contains("bold");
                ret.is_blink = value.contains("blink");
                ret.is_hidden = value.contains("hidden");
                ret.is_reverse = value.contains("reverse");
                ret.is_dimmed = value.contains("dimmed");
            }
            _ => {}
        }
    }

    CssStyle {
        style: ret,
        foreground_is_inherit,
        background_is_inherit,
    }
}

impl<'a, E, M> Clone for ElementView<'a, E, M> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, E, M> Copy for ElementView<'a, E, M> {}
