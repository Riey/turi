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
    #[inline(always)]
    fn try_mouse_down(&self) -> Option<Vec2> {
        None
    }

    #[inline(always)]
    fn try_mouse_up(&self) -> Option<Vec2> {
        None
    }

    #[inline(always)]
    fn try_drag(&self) -> Option<Vec2> {
        None
    }

    #[inline(always)]
    fn try_mouse(&self) -> Option<Vec2> {
        None
    }

    #[inline(always)]
    fn try_char(&self) -> Option<char> {
        None
    }

    #[inline(always)]
    fn try_ctrl_char(&self) -> Option<char> {
        None
    }

    #[inline(always)]
    fn try_enter(&self) -> bool {
        false
    }

    #[inline(always)]
    fn try_up(&self) -> bool {
        false
    }

    #[inline(always)]
    fn try_down(&self) -> bool {
        false
    }

    #[inline(always)]
    fn try_left(&self) -> bool {
        false
    }

    #[inline(always)]
    fn try_right(&self) -> bool {
        false
    }

    #[inline(always)]
    fn try_backspace(&self) -> bool {
        false
    }

    #[inline(always)]
    fn try_tab(&self) -> bool {
        false
    }
}
