use winit::{
    dpi::PhysicalPosition,
    event::{
        DeviceId,
        ElementState,
        ModifiersState,
        MouseButton,
        MouseScrollDelta,
        VirtualKeyCode,
        WindowEvent,
    },
};

use super::{
    EventLike,
    KeyEventLike,
    MouseEventLike,
};
use crate::vec2::Vec2;

enum ScrollDirection {
    Vertical(bool),
    Horizontal(bool),
}

impl From<MouseScrollDelta> for ScrollDirection {
    fn from(d: MouseScrollDelta) -> ScrollDirection {
        match d {
            MouseScrollDelta::LineDelta(x, y) if x == 0.0 => ScrollDirection::Vertical(y > 0.0),
            MouseScrollDelta::PixelDelta(PhysicalPosition { x, y }) if x == 0.0 => {
                ScrollDirection::Vertical(y > 0.0)
            }
            MouseScrollDelta::LineDelta(x, _) => ScrollDirection::Horizontal(x > 0.0),
            MouseScrollDelta::PixelDelta(PhysicalPosition { x, .. }) => {
                ScrollDirection::Horizontal(x > 0.0)
            }
        }
    }
}

pub struct WrapWindowEventState {
    ctrl:        bool,
    clicked:     bool,
    mouse_pos:   Vec2,
    letter_size: (f32, f32),
}

impl WrapWindowEventState {
    pub fn new(letter_size: (f32, f32)) -> Self {
        Self {
            ctrl: false,
            clicked: false,
            mouse_pos: Vec2::new(0, 0),
            letter_size,
        }
    }

    pub fn next_event(
        &mut self,
        e: WindowEvent<'static>,
    ) -> WrapWindowEvent {
        let mut drag = false;

        match e {
            WindowEvent::CursorMoved { position, .. } => {
                drag = self.clicked;
                self.mouse_pos = crate::util::calc_term_pos(
                    (position.x as _, position.y as _),
                    self.letter_size,
                );
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                ..
            } => {
                self.clicked = true;
            }
            WindowEvent::MouseInput {
                state: ElementState::Released,
                ..
            } => {
                self.clicked = false;
            }
            WindowEvent::ModifiersChanged(state) => {
                self.ctrl = state.ctrl();
            }
            _ => {}
        }

        WrapWindowEvent {
            e,
            ctrl: self.ctrl,
            drag,
            mouse_pos: self.mouse_pos,
            letter_size: self.letter_size,
        }
    }
}

#[derive(Clone)]
pub struct WrapWindowEvent {
    e:           WindowEvent<'static>,
    ctrl:        bool,
    drag:        bool,
    mouse_pos:   Vec2,
    letter_size: (f32, f32),
}

impl WrapWindowEvent {
    pub fn new(
        e: WindowEvent<'static>,
        ctrl: bool,
        drag: bool,
        mouse_pos: Vec2,
        letter_size: (f32, f32),
    ) -> Self {
        Self {
            e,
            ctrl,
            drag,
            mouse_pos,
            letter_size,
        }
    }
}

macro_rules! code_is {
    ($self:expr, $code:ident) => {
        if let WindowEvent::KeyboardInput { input, .. } = $self.e {
            input.virtual_keycode == Some(VirtualKeyCode::$code)
        } else {
            false
        }
    };
}

impl KeyEventLike for WrapWindowEvent {
    fn try_char(&self) -> Option<char> {
        if let WindowEvent::ReceivedCharacter(ch) = self.e {
            Some(ch)
        } else {
            None
        }
    }

    fn try_ctrl_char(&self) -> Option<char> {
        if let WindowEvent::ReceivedCharacter(ch) = self.e {
            if self.ctrl {
                Some(ch)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn try_enter(&self) -> bool {
        code_is!(self, Return)
    }

    fn try_up(&self) -> bool {
        code_is!(self, Up)
    }

    fn try_down(&self) -> bool {
        code_is!(self, Down)
    }

    fn try_left(&self) -> bool {
        code_is!(self, Left)
    }

    fn try_right(&self) -> bool {
        code_is!(self, Right)
    }

    fn try_backspace(&self) -> bool {
        code_is!(self, Back)
    }

    fn try_tab(&self) -> bool {
        code_is!(self, Tab)
    }
}

impl MouseEventLike for WrapWindowEvent {
    fn try_left_down(&self) -> Option<crate::vec2::Vec2> {
        if let WindowEvent::MouseInput {
            button: MouseButton::Left,
            state: ElementState::Pressed,
            ..
        } = self.e
        {
            Some(self.mouse_pos)
        } else {
            None
        }
    }

    fn try_left_up(&self) -> Option<crate::vec2::Vec2> {
        if let WindowEvent::MouseInput {
            button: MouseButton::Left,
            state: ElementState::Released,
            ..
        } = self.e
        {
            Some(self.mouse_pos)
        } else {
            None
        }
    }

    fn try_drag(&self) -> Option<crate::vec2::Vec2> {
        if self.drag {
            if matches!(self.e, WindowEvent::CursorMoved {..}) {
                Some(self.mouse_pos)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn try_scroll_up(&self) -> Option<crate::vec2::Vec2> {
        if let WindowEvent::MouseWheel { delta, .. } = self.e {
            match ScrollDirection::from(delta) {
                ScrollDirection::Vertical(true) => Some(self.mouse_pos),
                _ => None,
            }
        } else {
            None
        }
    }

    fn try_scroll_down(&self) -> Option<crate::vec2::Vec2> {
        if let WindowEvent::MouseWheel { delta, .. } = self.e {
            match ScrollDirection::from(delta) {
                ScrollDirection::Vertical(false) => Some(self.mouse_pos),
                _ => None,
            }
        } else {
            None
        }
    }

    fn pos(&self) -> crate::vec2::Vec2 {
        self.mouse_pos
    }

    fn map_pos(
        &mut self,
        f: impl FnOnce(crate::vec2::Vec2) -> crate::vec2::Vec2,
    ) {
        self.mouse_pos = f(self.mouse_pos);
    }

    fn filter_map_pos(
        &mut self,
        f: impl FnOnce(crate::vec2::Vec2) -> Option<crate::vec2::Vec2>,
    ) -> bool {
        self.mouse_pos = match f(self.mouse_pos) {
            Some(pos) => pos,
            None => return false,
        };

        true
    }

    fn from_left_down(pos: crate::vec2::Vec2) -> Self {
        Self {
            e:           WindowEvent::MouseInput {
                state:     ElementState::Pressed,
                button:    MouseButton::Left,
                device_id: unsafe { DeviceId::dummy() },
                modifiers: ModifiersState::empty(),
            },
            ctrl:        false,
            drag:        false,
            mouse_pos:   pos,
            letter_size: (1.0, 1.0),
        }
    }

    fn from_left_up(pos: crate::vec2::Vec2) -> Self {
        Self {
            e:           WindowEvent::MouseInput {
                state:     ElementState::Released,
                button:    MouseButton::Left,
                device_id: unsafe { DeviceId::dummy() },
                modifiers: ModifiersState::empty(),
            },
            ctrl:        false,
            drag:        false,
            mouse_pos:   pos,
            letter_size: (1.0, 1.0),
        }
    }
}

impl EventLike for WrapWindowEvent {
    type KeyEvent = Self;
    type MouseEvent = Self;

    fn try_mouse(&self) -> Option<&Self::MouseEvent> {
        match self.e {
            WindowEvent::MouseWheel { .. }
            | WindowEvent::MouseInput { .. }
            | WindowEvent::CursorMoved { .. } => Some(self),
            _ => None,
        }
    }

    fn try_mouse_mut(&mut self) -> Option<&mut Self::MouseEvent> {
        match self.e {
            WindowEvent::MouseWheel { .. }
            | WindowEvent::MouseInput { .. }
            | WindowEvent::CursorMoved { .. } => Some(self),
            _ => None,
        }
    }

    fn try_key(&self) -> Option<&Self::KeyEvent> {
        match self.e {
            WindowEvent::KeyboardInput { .. } | WindowEvent::ReceivedCharacter(..) => Some(self),
            _ => None,
        }
    }

    fn try_key_mut(&mut self) -> Option<&mut Self::KeyEvent> {
        match self.e {
            WindowEvent::KeyboardInput { .. } | WindowEvent::ReceivedCharacter(..) => Some(self),
            _ => None,
        }
    }

    fn try_resize(&self) -> Option<crate::vec2::Vec2> {
        match self.e {
            WindowEvent::Resized(size) => {
                let width = size.width as f32 / self.letter_size.0;
                let height = size.height as f32 / self.letter_size.1;
                Some(Vec2::new(width as _, height as _))
            }
            _ => None,
        }
    }
}
