use crate::{
    backend::Backend,
    event::Event,
    style::AnsiStyle as Style,
    vec2::Vec2,
};

use std::{time::Duration, iter};
use unicode_width::UnicodeWidthStr;

pub struct TestBackend<I: Iterator<Item = Event>> {
    lines: Vec<String>,
    style: Style,
    size:  Vec2,
    events: I,
}

impl<I: Iterator<Item = Event>> TestBackend<I> {
    pub fn new(size: Vec2, events: I) -> Self {
        Self {
            lines: iter::repeat_with(|| " ".repeat(size.x as usize))
                .take(size.y as usize)
                .collect(),
            style: Style::default(),
            events,
            size,
        }
    }

    pub fn lines(&self) -> &[String] {
        &self.lines[..]
    }
}

impl<I: Iterator<Item = Event>> Backend for TestBackend<I> {
    #[inline]
    fn clear(&mut self) {
        for line in &mut self.lines {
            line.clear();
            for _ in 0..self.size.x {
                line.push(' ');
            }
        }
    }

    #[inline]
    fn size(&self) -> Vec2 {
        self.size
    }

    #[inline]
    fn set_style(
        &mut self,
        style: Style,
    ) {
        self.style = style;
    }

    #[inline]
    fn style(&self) -> Style {
        self.style
    }

    #[inline]
    fn print_at(
        &mut self,
        pos: Vec2,
        text: &str,
    ) {
        let line = &mut self.lines[pos.y as usize];
        let width = text.width();

        let (mut start, start_left) = crate::util::find_str_width_pos(line, pos.x as usize);
        let (end, _end_left) = crate::util::find_str_width_pos(line, pos.x as usize + width);

        if start_left > 0 {
            let mut start_blank = start + start_left;
            while !line.is_char_boundary(start_blank) {
                start_blank += 1;
            }

            line.replace_range(start..start_blank, " ".repeat(start_blank - start).as_str());

            start += start_left;
        }

        line.replace_range(start..end, text);
    }

    #[inline]
    fn flush(&mut self) {}

    #[inline]
    fn poll_event(&mut self, _: Duration) -> Option<Event> {
        self.events.next()
    }
}

#[test]
fn test_backend_test() {
    let mut backend = TestBackend::new(Vec2::new(10, 5), std::iter::empty());
    backend.print_at(Vec2::new(2, 2), "가나다");
    backend.print_at(Vec2::new(2, 1), "ABC");
    backend.print_at(Vec2::new(7, 2), "라");

    pretty_assertions::assert_eq!(backend.lines(), &[
        "          ",
        "  ABC     ",
        "  가나 라 ",
        "          ",
        "          ",
    ]);
}
