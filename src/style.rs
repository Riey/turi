use enum_map::{
    Enum,
    EnumMap,
};
use std::mem::MaybeUninit;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaletteColor<Label> {
    Background,
    View,
    Primary,
    Title,
    Highlight,
    HighlightInactive,
    Custom(Label),
}

pub struct LabelArray<T, A>([T; 6], A);

impl<Label, T> Enum<T> for PaletteColor<Label>
where
    Label: Enum<T>,
    T: Sized,
    Label::Array: Sized,
{
    type Array = LabelArray<T, Label::Array>;

    const POSSIBLE_VALUES: usize = 6 + Label::POSSIBLE_VALUES;

    #[inline]
    fn slice(array: &Self::Array) -> &[T] {
        unsafe { std::slice::from_raw_parts(array as *const _ as *const _, Self::POSSIBLE_VALUES) }
    }

    #[inline]
    fn slice_mut(array: &mut Self::Array) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(array as *mut _ as *mut _, Self::POSSIBLE_VALUES) }
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
            x => PaletteColor::Custom(Label::from_usize(x - 6)),
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
            PaletteColor::Custom(labal) => labal.to_usize() + 6,
        }
    }

    #[inline]
    fn from_function<F: FnMut(Self) -> T>(mut f: F) -> Self::Array {
        unsafe {
            let mut arr = MaybeUninit::<Self::Array>::uninit();

            (*arr.as_mut_ptr()).0[0] = f(PaletteColor::Background);
            (*arr.as_mut_ptr()).0[1] = f(PaletteColor::View);
            (*arr.as_mut_ptr()).0[2] = f(PaletteColor::Primary);
            (*arr.as_mut_ptr()).0[3] = f(PaletteColor::Title);
            (*arr.as_mut_ptr()).0[4] = f(PaletteColor::Highlight);
            (*arr.as_mut_ptr()).0[5] = f(PaletteColor::HighlightInactive);

            let label_arr = Label::from_function(|label| f(PaletteColor::Custom(label)));

            (*arr.as_mut_ptr()).1 = label_arr;

            arr.assume_init()
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BaseColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    Ansi(u8),
    Rgb(u8, u8, u8),
    Reset,
}

impl Default for BaseColor {
    fn default() -> Self {
        BaseColor::Reset
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Color<Label> {
    Base(BaseColor),
    Palette(PaletteColor<Label>),
}

pub struct Theme<Label: Enum<BaseColor>> {
    palette: EnumMap<PaletteColor<Label>, BaseColor>,
}

impl<Label: Enum<BaseColor>> Theme<Label> {
    pub fn new() -> Self {
        Self {
            palette: EnumMap::new(),
        }
    }

    #[inline]
    pub fn resolve_palette(
        &self,
        palette: PaletteColor<Label>,
    ) -> BaseColor {
        self.palette[palette]
    }

    #[inline]
    pub fn resolve_color(
        &self,
        color: Color<Label>,
    ) -> BaseColor {
        match color {
            Color::Base(base) => base,
            Color::Palette(pallete) => self.resolve_palette(pallete),
        }
    }
}
