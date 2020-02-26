use crate::vec2::Vec2;

pub trait EventLike {
    fn try_mouse_down(&self) -> Option<Vec2>;
    fn try_mouse_up(&self) -> Option<Vec2>;
    fn try_drag(&self) -> Option<Vec2>;
    fn try_mouse(&self) -> Option<Vec2>;
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
    fn try_mouse_down(&self) -> Option<Vec2> {
        None
    }

    #[inline]
    fn try_mouse_up(&self) -> Option<Vec2> {
        None
    }

    #[inline]
    fn try_drag(&self) -> Option<Vec2> {
        None
    }

    #[inline]
    fn try_mouse(&self) -> Option<Vec2> {
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
