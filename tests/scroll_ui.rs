use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
    KeyModifiers,
    MouseButton,
    MouseEvent,
};
use turi::{
    executor,
    orientation::Orientation,
    view::View,
    views::{
        LinearView,
        TextView,
    },
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

#[test]
fn horizontal_scroll_mouse_down_linear_view() {
    executor::test(
        &mut LinearView::vertical()
            .child(TextView::new("123456").scrollable(Orientation::Horizontal)),
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

#[test]
fn horizontal_scroll_key_right() {
    executor::test(
        &mut TextView::new("123456").scrollable(Orientation::Horizontal),
        vec![
            Event::Key(KeyEvent {
                code:      KeyCode::Right,
                modifiers: KeyModifiers::empty(),
            }),
            Event::Key(KeyEvent {
                code:      KeyCode::Right,
                modifiers: KeyModifiers::empty(),
            }),
        ],
        (4, 2).into(),
        |lines| {
            assert_eq!(lines, &["2345", "──░─",]);
        },
    )
}

#[test]
fn horizontal_scroll_key_right_linear_view() {
    executor::test(
        &mut LinearView::vertical()
            .child(TextView::new("123456").scrollable(Orientation::Horizontal)),
        vec![
            Event::Key(KeyEvent {
                code:      KeyCode::Right,
                modifiers: KeyModifiers::empty(),
            }),
            Event::Key(KeyEvent {
                code:      KeyCode::Right,
                modifiers: KeyModifiers::empty(),
            }),
        ],
        (4, 2).into(),
        |lines| {
            assert_eq!(lines, &["2345", "──░─",]);
        },
    )
}
#[test]
fn horizontal_scroll_key_right_linear_view_two_childs() {
    executor::test(
        &mut LinearView::vertical()
            .focus(1)
            .child(TextView::new("ABC"))
            .child(TextView::new("123456").scrollable(Orientation::Horizontal)),
        vec![
            Event::Key(KeyEvent {
                code:      KeyCode::Right,
                modifiers: KeyModifiers::empty(),
            }),
            Event::Key(KeyEvent {
                code:      KeyCode::Right,
                modifiers: KeyModifiers::empty(),
            }),
        ],
        (4, 3).into(),
        |lines| {
            assert_eq!(lines, &["ABC ", "2345", "──░─",]);
        },
    )
}
