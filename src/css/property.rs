use crate::css::{
    AnsiStyle,
    Color,
    CssFontStyle,
    CssRect,
    CssSize,
    CssVal,
};
use enumset::EnumSet;

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
        parent_style: AnsiStyle,
    ) -> AnsiStyle {
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
