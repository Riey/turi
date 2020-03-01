use crossterm::event::{
    Event,
    KeyModifiers,
    MouseButton,
    MouseEvent,
};
use turi::{
    executor,
    orientation::Orientation,
    view::View,
    views::TextView,
};

#[test]
fn horizontal_scroll_mouse_down() {
    executor::test(
        &mut TextView::new("123456").scrollable(Orientation::Horizontal),
        vec![Event::Mouse(MouseEvent::Down(
            MouseButton::Left,
            2,
            1,
            KeyModifiers::empty(),
        ))],
        (4, 2).into(),
        |lines| {
            assert_eq!(lines, &["2345", "──█─",]);
        },
    )
}
