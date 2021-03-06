use crate::{
    never::Never,
    printer::Printer,
    style::Style,
    vec2::Vec2,
    view::View,
};
use std::marker::PhantomData;
use unicode_width::UnicodeWidthStr;

pub struct ParagraphView<S, E> {
    lines:   Vec<String>,
    width:   usize,
    _marker: PhantomData<(S, E)>,
}

impl<S, E> ParagraphView<S, E> {
    pub fn new() -> Self {
        let mut lines = Vec::with_capacity(10);
        lines.push(String::with_capacity(100));
        Self {
            lines,
            width: 0,
            _marker: PhantomData,
        }
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

impl<S, E> View<S, E> for ParagraphView<S, E> {
    type Message = Never;

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        printer.with_style(Style::view(), |printer| {
            for (y, line) in self.lines.iter().enumerate() {
                printer.print((0, y as u16), line);
            }
        });
    }

    fn layout(
        &mut self,
        _size: Vec2,
    ) {
    }

    fn desired_size(&self) -> Vec2 {
        Vec2::new(self.width as u16, self.lines.len() as u16)
    }

    fn on_event(
        &mut self,
        _: &mut S,
        _: E,
    ) -> Option<Never> {
        None
    }
}
