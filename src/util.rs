use unicode_width::UnicodeWidthChar;

#[inline]
pub fn slice_str_with_width(text: &str, mut width: usize) -> (&str, &str, usize) {
    for (i, ch) in text.char_indices() {
        match width.checked_sub(ch.width().unwrap_or(0)) {
            Some(new_width) => {
                width = new_width;
            }
            None => {
                return unsafe {
                    (text.get_unchecked(..i), text.get_unchecked(i..), width)
                };
            }
        }
    }

    (text, "", 0)
}
