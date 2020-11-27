#[cfg(feature = "crossterm-event")]
mod crossterm;

#[cfg(feature = "winit-event")]
mod winit;

#[cfg(feature = "winit-event")]
pub use self::winit::{
    WrapWindowEvent,
    WrapWindowEventState,
};

use crate::vec2::Vec2;

pub trait MouseEventLike: Sized {
    fn try_left_down(&self) -> Option<Vec2>;
    fn try_left_up(&self) -> Option<Vec2>;
    fn try_drag(&self) -> Option<Vec2>;
    fn try_scroll_up(&self) -> Option<Vec2>;
    fn try_scroll_down(&self) -> Option<Vec2>;
    fn pos(&self) -> Vec2;
    fn map_pos(
        &mut self,
        f: impl FnOnce(Vec2) -> Vec2,
    );
    fn filter_map_pos(
        &mut self,
        f: impl FnOnce(Vec2) -> Option<Vec2>,
    ) -> bool;

    fn from_left_down(pos: Vec2) -> Self;
    fn from_left_up(pos: Vec2) -> Self;
}

pub trait KeyEventLike {
    fn try_char(&self) -> Option<char>;
    fn try_ctrl_char(&self) -> Option<char>;
    fn try_enter(&self) -> bool;
    fn try_up(&self) -> bool;
    fn try_down(&self) -> bool;
    fn try_left(&self) -> bool;
    fn try_right(&self) -> bool;
    fn try_backspace(&self) -> bool;
    fn try_tab(&self) -> bool;
}

pub trait EventLike: Sized {
    type MouseEvent: MouseEventLike + Clone;
    type KeyEvent: KeyEventLike + Clone;

    fn try_mouse(&self) -> Option<&Self::MouseEvent>;
    fn try_mouse_mut(&mut self) -> Option<&mut Self::MouseEvent>;
    fn try_key(&self) -> Option<&Self::KeyEvent>;
    fn try_key_mut(&mut self) -> Option<&mut Self::KeyEvent>;
    fn try_resize(&self) -> Option<Vec2>;
}
