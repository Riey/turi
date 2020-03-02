#[cfg(any(unix, windows, target_os = "wasi"))]
use std::fs::File;
#[cfg(any(unix, windows))]
use std::io::{
    self,
    Write,
};
#[cfg(any(unix, windows))]
use std::mem::ManuallyDrop;

use unicode_width::UnicodeWidthChar;

#[cfg(any(unix, target_os = "wasi"))]
pub fn get_tty_file() -> File {
    File::create("/dev/tty").unwrap()
}

#[cfg(any(unix, windows))]
pub struct RawStdout(ManuallyDrop<File>);

#[cfg(any(unix, windows))]
impl Write for RawStdout {
    #[inline]
    fn write_vectored(
        &mut self,
        bufs: &[io::IoSlice],
    ) -> io::Result<usize> {
        self.0.write_vectored(bufs)
    }

    #[inline]
    fn write_all(
        &mut self,
        buf: &[u8],
    ) -> io::Result<()> {
        self.0.write_all(buf)
    }

    #[inline]
    fn write_fmt(
        &mut self,
        fmt: std::fmt::Arguments,
    ) -> io::Result<()> {
        self.0.write_fmt(fmt)
    }

    #[inline]
    fn write(
        &mut self,
        buf: &[u8],
    ) -> io::Result<usize> {
        self.0.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

#[cfg(any(unix, windows))]
pub fn get_raw_stdout_file() -> RawStdout {
    #[cfg(unix)]
    {
        use std::os::unix::io::FromRawFd;
        unsafe {
            RawStdout(ManuallyDrop::new(FromRawFd::from_raw_fd(
                libc::STDOUT_FILENO,
            )))
        }
    }

    #[cfg(windows)]
    {
        use std::os::windows::io::FromRawHandle;
        unsafe {
            RawStdout(ManuallyDrop::new(FromRawHandle::from_raw_handle(
                winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE),
            )))
        }
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
