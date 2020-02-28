use enum_map::{
    Enum,
    EnumMap,
};
use enumset::{EnumSet, EnumSetType};
use std::mem::MaybeUninit;

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

impl<T> Enum<T> for PaletteColor
{
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BaseColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasicColor {
    Light(BaseColor),
    Dark(BaseColor),
    Ansi(u8),
    Rgb(u8, u8, u8),
    Reset,
}

impl Default for BasicColor {
    fn default() -> Self {
        BasicColor::Reset
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Basic(BasicColor),
    Palette(PaletteColor),
}

impl Default for Color {
    #[inline]
    fn default() -> Self { Color::Basic(BasicColor::Reset) }
}

#[derive(EnumSetType, Debug)]
pub enum Effect {
    Bold,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    StrikeThrough,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Style {
    pub fg: BasicColor,
    pub bg: BasicColor,
    pub effects: EnumSet<Effect>,
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    palette: EnumMap<PaletteColor, BasicColor>,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            palette: EnumMap::new(),
        }
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
}
