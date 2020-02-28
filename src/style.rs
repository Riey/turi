use enum_map::{
    enum_map,
    Enum,
    EnumMap,
};
use enumset::{
    EnumSet,
    EnumSetType,
};
use std::mem::MaybeUninit;

pub use ansi_term::{
    Color as AnsiColor,
    Style as AnsiStyle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaletteColor {
    Background,
    View,
    Primary,
    Title,
    Highlight,
    HighlightInactive,
    Custom(u8),
}

impl<T> Enum<T> for PaletteColor {
    type Array = [T; 6 + 256];

    const POSSIBLE_VALUES: usize = 6 + 256;

    #[inline]
    fn slice(array: &Self::Array) -> &[T] {
        &array[..]
    }

    #[inline]
    fn slice_mut(array: &mut Self::Array) -> &mut [T] {
        &mut array[..]
    }

    #[inline]
    fn from_usize(value: usize) -> Self {
        match value {
            0 => PaletteColor::Background,
            1 => PaletteColor::View,
            2 => PaletteColor::Primary,
            3 => PaletteColor::Title,
            4 => PaletteColor::Highlight,
            5 => PaletteColor::HighlightInactive,
            x => PaletteColor::Custom((x - 6) as u8),
        }
    }

    #[inline]
    fn to_usize(self) -> usize {
        match self {
            PaletteColor::Background => 0,
            PaletteColor::View => 1,
            PaletteColor::Primary => 2,
            PaletteColor::Title => 3,
            PaletteColor::Highlight => 4,
            PaletteColor::HighlightInactive => 5,
            PaletteColor::Custom(labal) => labal as usize + 6,
        }
    }

    #[inline]
    fn from_function<F: FnMut(Self) -> T>(mut f: F) -> Self::Array {
        unsafe {
            let mut arr = MaybeUninit::<Self::Array>::uninit();

            (*arr.as_mut_ptr())[0] = f(PaletteColor::Background);
            (*arr.as_mut_ptr())[1] = f(PaletteColor::View);
            (*arr.as_mut_ptr())[2] = f(PaletteColor::Primary);
            (*arr.as_mut_ptr())[3] = f(PaletteColor::Title);
            (*arr.as_mut_ptr())[4] = f(PaletteColor::Highlight);
            (*arr.as_mut_ptr())[5] = f(PaletteColor::HighlightInactive);

            for i in 0..256 {
                (*arr.as_mut_ptr())[i + 6] = f(PaletteColor::Custom(i as u8));
            }

            arr.assume_init()
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BasicColor {
    Ansi(AnsiColor),
    Reset,
}

impl From<BasicColor> for Option<AnsiColor> {
    #[inline]
    fn from(c: BasicColor) -> Self {
        match c {
            BasicColor::Reset => None,
            BasicColor::Ansi(ansi) => Some(ansi),
        }
    }
}

impl From<Option<AnsiColor>> for BasicColor {
    #[inline]
    fn from(c: Option<AnsiColor>) -> Self {
        match c {
            Some(ansi) => ansi.into(),
            None => BasicColor::Reset,
        }
    }
}

impl From<AnsiColor> for BasicColor {
    #[inline]
    fn from(c: AnsiColor) -> Self {
        BasicColor::Ansi(c)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Basic(BasicColor),
    Palette(PaletteColor),
}

impl Default for Color {
    #[inline]
    fn default() -> Self {
        Color::Basic(BasicColor::Reset)
    }
}

#[derive(EnumSetType, Debug)]
pub enum Effect {
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    StrikeThrough,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Style {
    pub fg:      Color,
    pub bg:      Color,
    pub effects: EnumSet<Effect>,
}

impl Style {
    #[inline]
    pub fn outline() -> Self {
        Self {
            fg:      Color::Palette(PaletteColor::Primary),
            bg:      Color::Palette(PaletteColor::Background),
            effects: EnumSet::empty(),
        }
    }

    #[inline]
    pub fn title() -> Self {
        Self {
            fg:      Color::Palette(PaletteColor::Title),
            bg:      Color::Palette(PaletteColor::Background),
            effects: EnumSet::empty(),
        }
    }

    #[inline]
    pub fn view() -> Self {
        Self {
            fg:      Color::Palette(PaletteColor::Primary),
            bg:      Color::Palette(PaletteColor::View),
            effects: EnumSet::empty(),
        }
    }

    #[inline]
    pub fn highlight() -> Self {
        Self {
            fg:      Color::Palette(PaletteColor::Highlight),
            bg:      Color::Palette(PaletteColor::View),
            effects: Effect::Reverse.into(),
        }
    }

    #[inline]
    pub fn highlight_inactive() -> Self {
        Self {
            fg:      Color::Palette(PaletteColor::HighlightInactive),
            bg:      Color::Palette(PaletteColor::View),
            effects: Effect::Reverse.into(),
        }
    }

    #[inline]
    pub fn fg(
        mut self,
        fg: Color,
    ) -> Self {
        self.fg = fg;
        self
    }

    #[inline]
    pub fn bg(
        mut self,
        bg: Color,
    ) -> Self {
        self.bg = bg;
        self
    }

    #[inline]
    pub fn effects(
        mut self,
        effects: impl Into<EnumSet<Effect>>,
    ) -> Self {
        self.effects |= effects;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    palette: EnumMap<PaletteColor, BasicColor>,
}

impl Default for Theme {
    fn default() -> Self {
        Self::new(enum_map! {
            PaletteColor::View => BasicColor::Reset,
            PaletteColor::Background => BasicColor::Reset,
            PaletteColor::Primary => BasicColor::Ansi(AnsiColor::White),
            PaletteColor::Title => BasicColor::Ansi(AnsiColor::Cyan),
            PaletteColor::Highlight => BasicColor::Ansi(AnsiColor::Yellow),
            PaletteColor::HighlightInactive => BasicColor::Ansi(AnsiColor::Black),
            PaletteColor::Custom(_) => BasicColor::Reset,
        })
    }
}

impl Theme {
    pub fn new(palette: EnumMap<PaletteColor, BasicColor>) -> Self {
        Self { palette }
    }

    #[inline]
    pub fn resolve_palette(
        &self,
        palette: PaletteColor,
    ) -> BasicColor {
        self.palette[palette]
    }

    #[inline]
    pub fn resolve_color(
        &self,
        color: Color,
    ) -> BasicColor {
        match color {
            Color::Basic(basic) => basic,
            Color::Palette(pallete) => self.resolve_palette(pallete),
        }
    }

    #[inline]
    pub fn resolve_style(
        &self,
        style: &Style,
    ) -> AnsiStyle {
        AnsiStyle {
            foreground:       self.resolve_color(style.fg).into(),
            background:       self.resolve_color(style.bg).into(),
            is_bold:          style.effects.contains(Effect::Bold),
            is_blink:         style.effects.contains(Effect::Blink),
            is_dimmed:        style.effects.contains(Effect::Dim),
            is_italic:        style.effects.contains(Effect::Italic),
            is_underline:     style.effects.contains(Effect::Underline),
            is_reverse:       style.effects.contains(Effect::Reverse),
            is_hidden:        style.effects.contains(Effect::Hidden),
            is_strikethrough: style.effects.contains(Effect::StrikeThrough),
        }
    }
}
