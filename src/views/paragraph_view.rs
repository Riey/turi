use crate::{
    printer::Printer,
    vec2::Vec2,
    view::{
        ScrollableView,
        View,
    },
};
use std::slice::SliceIndex;
use unicode_width::{
    UnicodeWidthChar,
    UnicodeWidthStr,
};

pub struct ParagraphView {
    lines: Vec<String>,
    width: usize,
}

impl ParagraphView {
    pub fn new() -> Self {
        let mut lines = Vec::with_capacity(10);
        lines.push(String::with_capacity(100));
        Self { lines, width: 0 }
    }

    pub fn append(
        &mut self,
        text: &str,
    ) {
        let mut lines = text.split('\n');

        let first_line = match lines.next() {
            Some(first) => first,
            None => {
                return;
            }
        };

        let last_line = self.lines.last_mut().unwrap();
        last_line.push_str(first_line);

        self.width = self.width.max(last_line.width());

        for line in lines {
            self.push_line(line);
        }
    }

    pub fn new_line(&mut self) {
        self.lines.push(String::with_capacity(50));
    }

    pub fn append_line(
        &mut self,
        text: &str,
    ) {
        self.append(text);
        self.new_line();
    }

    pub fn push_line(
        &mut self,
        line: impl Into<String>,
    ) {
        let line = line.into();
        self.width = self.width.max(line.width());
        self.lines.push(line);
    }
}

impl View for ParagraphView {
    fn render(
        &self,
        printer: &mut Printer,
    ) {
        for (y, line) in self.lines.iter().enumerate() {
            printer.print((0, y as u16), line);
        }
    }

    fn layout(
        &mut self,
        _size: Vec2,
    ) {
    }

    fn desired_size(&self) -> Vec2 {
        Vec2::new(self.width as u16, self.lines.len() as u16)
    }
}

fn scroll_horizontal_render_impl(
    lines: &Vec<String>,
    pos: usize,
    printer: &mut Printer,
    idx: impl SliceIndex<[String], Output = [String]>,
) {
    for (y, line) in lines[idx].iter().enumerate() {
        let mut left = pos;

        for (idx, ch) in line.char_indices() {
            let width = ch.width().unwrap_or(0);
            match left.checked_sub(width) {
                Some(num) => {
                    left = num;
                }
                None => {
                    printer.print((0, y as u16), &line[idx..]);
                    break;
                }
            }
        }
    }
}

impl ScrollableView for ParagraphView {
    fn scroll_vertical_render(
        &self,
        pos: u16,
        printer: &mut Printer,
    ) {
        for (y, line) in self.lines[pos as usize..(pos + printer.bound().h()) as usize]
            .iter()
            .enumerate()
        {
            printer.print((0, y as u16), line);
        }
    }

    fn scroll_horizontal_render(
        &self,
        pos: u16,
        printer: &mut Printer,
    ) {
        scroll_horizontal_render_impl(&self.lines, pos as usize, printer, ..);
    }

    fn scroll_both_render(
        &self,
        pos: Vec2,
        printer: &mut Printer,
    ) {
        scroll_horizontal_render_impl(
            &self.lines,
            pos.x as usize,
            printer,
            pos.y as usize..(pos.y + printer.bound().h()) as usize,
        );
    }
}
