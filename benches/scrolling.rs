#![feature(test)]

extern crate test;

use enumset::EnumSet;
use turi::{
    event::{
        Event,
        KeyModifiers,
        MouseButton,
        MouseEvent,
    },
    executor,
    orientation::Orientation,
    vec2::Vec2,
    view::View,
    views::TextView,
};

#[bench]
fn crossterm_scroll_bench(b: &mut test::Bencher) {
    let mut buf = Vec::with_capacity(1024 * 1024);
    let mut events = Vec::with_capacity(1024);
    let mut view = TextView::new("1234567890".repeat(10)).scrollable(Orientation::Horizontal);

    for _ in 0..512 {
        events.push(Event::Mouse(MouseEvent::Down(
            MouseButton::Left,
            (50, 9).into(),
        )));
        events.push(Event::Mouse(MouseEvent::Down(
            MouseButton::Left,
            (45, 9).into(),
        )));
    }

    b.iter(|| {
        let mut backend = turi::backend::CrosstermBackend::new(&mut buf, Vec2::new(40, 10));
        executor::bench(&mut backend, &mut view, events.iter().copied());
        buf.clear();
    });
}
