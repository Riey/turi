use crate::vec2::Vec2;
use enumset::{
    EnumSet,
    EnumSetType,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MouseEvent {
    Down(MouseButton, Vec2),
    Up(MouseButton, Vec2),
    Drag(MouseButton, Vec2),
    ScrollUp(Vec2),
    ScrollDown(Vec2),
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyCode {
    Enter,
    Space,
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Tab,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyEventType {
    Key(KeyCode),
    Char(char),
}

#[derive(EnumSetType, Debug)]
pub enum KeyModifiers {
    Control,
    Shift,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyEvent(pub KeyEventType, pub EnumSet<KeyModifiers>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Event {
    Mouse(MouseEvent),
    Key(KeyEvent),
}
