use enumflags2::BitFlags;

#[derive(Clone, Copy, Debug, BitFlags)]
pub enum Modifiers {
    Bold = 0x1,
    Italic = 0x2,
    Reverse = 0x4,
}
