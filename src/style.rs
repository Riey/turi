use crate::view::{
    Tag,
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

pub struct ElementView<'a, 'p, E, M> {
    parent: Option<&'p Self>,
    pos:    usize,
    view:   View<'a, E, M>,
}

impl<'a, 'p, E, M> ElementView<'a, 'p, E, M> {
    pub fn with_view(view: View<'a, E, M>) -> ElementView<'a, 'static, E, M> {
        ElementView {
            parent: None,
            pos: 0,
            view,
        }
    }

    pub fn with_parent(
        parent: &'p Self,
        pos: usize,
    ) -> Option<Self> {
        Some(Self {
            parent: Some(parent),
            pos,
            view: parent.view.children().get(pos).cloned()?,
        })
    }
}

impl<'a, 'p, E, M> Element for ElementView<'a, 'p, E, M> {
    fn parent_element(&self) -> Option<Self> {
        self.parent.copied()
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        Self::with_parent(self.parent?, self.pos.checked_sub(1)?)
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
            style:    todo!(),
        }
    }
}

pub struct StyleSheet<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> StyleSheet<'a> {
    pub fn calc_style<E, M>(
        &self,
        parent_style: Style,
        view: &ElementView<'_, 'a, E, M>,
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
        "transparent" | "inherit" => return None,
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

fn convert_declar<'a>(declarations: Vec<Declaration<'a>>) -> Style {
    let mut ret = Style::new();

    for Declaration { name, value, .. } in declarations {
        match name {
            "color" => {
                ret.foreground = convert_color(value);
            }
            "background" => {
                ret.background = convert_color(value);
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

    ret
}

impl<'a, 'p, E, M> Clone for ElementView<'a, 'p, E, M> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, 'p, E, M> Copy for ElementView<'a, 'p, E, M> {}
