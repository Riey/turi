#[cfg(any(unix, windows, target_os = "wasi"))]
use std::fs::File;
use unicode_width::UnicodeWidthChar;

#[cfg(any(unix, target_os = "wasi"))]
pub fn get_tty_file() -> File {
    File::create("/dev/tty").unwrap()
}

#[cfg(any(unix, windows))]
pub unsafe fn get_raw_stdout_file() -> File {
    #[cfg(unix)]
    {
        use std::os::unix::io::FromRawFd;
        FromRawFd::from_raw_fd(libc::STDOUT_FILENO)
    }

    #[cfg(windows)]
    {
        use std::os::windows::io::FromRawHandle;
        FromRawHandle::from_raw_handle(winapi::um::processenv::GetStdHandle(
            winapi::um::winbase::STD_OUTPUT_HANDLE,
        ))
    }
}

#[inline]
pub fn slice_str_with_width(
    text: &str,
    width: usize,
) -> (&str, &str, usize) {
    let (i, width) = find_str_width_pos(text, width);
    unsafe { (text.get_unchecked(..i), text.get_unchecked(i..), width) }
}

#[inline]
pub fn find_str_width_pos(
    text: &str,
    mut width: usize,
) -> (usize, usize) {
    for (i, ch) in text.char_indices() {
        match width.checked_sub(ch.width().unwrap_or(0)) {
            Some(new_width) => {
                width = new_width;
            }
            None => {
                return (i, width);
            }
        }
    }

    (text.len(), width)
}

#[test]
fn slice_test() {
    assert_eq!(slice_str_with_width("123456", 3), ("123", "456", 0));
    assert_eq!(slice_str_with_width("abcdefghi", 5), ("abcde", "fghi", 0));
}

#[test]
fn slice_left_test() {
    assert_eq!(slice_str_with_width("가나다라", 3), ("가", "나다라", 1));
}
