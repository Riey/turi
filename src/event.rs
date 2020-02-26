use crate::vec2::Vec2;

pub trait EventLike: Sized {
    fn try_left_down(&self) -> Option<Vec2>;
    fn try_left_up(&self) -> Option<Vec2>;

    fn from_left_down(pos: Vec2) -> Self;
    fn from_left_up(pos: Vec2) -> Self;
    fn try_drag(&self) -> Option<Vec2>;
    fn try_mouse(&self) -> Option<Vec2>;
    fn try_map_mouse<F>(
        &self,
        f: F,
    ) -> Option<Self>
    where
        F: FnOnce(Vec2) -> Vec2;
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

#[derive(Clone, Copy, Debug)]
pub struct NoneEvent;

impl EventLike for NoneEvent {
    #[inline]
    fn try_left_down(&self) -> Option<Vec2> {
        None
    }

    #[inline]
    fn try_left_up(&self) -> Option<Vec2> {
        None
    }

    #[inline]
    fn try_drag(&self) -> Option<Vec2> {
        None
    }

    #[inline]
    fn from_left_down(_: Vec2) -> Self {
        Self
    }

    #[inline]
    fn from_left_up(_: Vec2) -> Self {
        Self
    }

    #[inline]
    fn try_mouse(&self) -> Option<Vec2> {
        None
    }

    #[inline]
    fn try_map_mouse<F>(
        &self,
        _f: F,
    ) -> Option<Self>
    where
        F: FnOnce(Vec2) -> Vec2,
    {
        None
    }

    #[inline]
    fn try_char(&self) -> Option<char> {
        None
    }

    #[inline]
    fn try_ctrl_char(&self) -> Option<char> {
        None
    }

    #[inline]
    fn try_enter(&self) -> bool {
        false
    }

    #[inline]
    fn try_up(&self) -> bool {
        false
    }

    #[inline]
    fn try_down(&self) -> bool {
        false
    }

    #[inline]
    fn try_left(&self) -> bool {
        false
    }

    #[inline]
    fn try_right(&self) -> bool {
        false
    }

    #[inline]
    fn try_backspace(&self) -> bool {
        false
    }

    #[inline]
    fn try_tab(&self) -> bool {
        false
    }
}
