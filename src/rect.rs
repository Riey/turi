use crate::vec2::Vec2;
use derive_more::From;

#[derive(Clone, Copy, Debug, Eq, PartialEq, From)]
pub struct Rect {
    start: Vec2,
    size: Vec2,
}

impl Rect {
    pub fn new(start: impl Into<Vec2>, size: impl Into<Vec2>) -> Self {
        Self {
            start: start.into(),
            size: size.into(),
        }
    }

    pub fn contains(self, p: impl Into<Vec2>) -> bool {
        let p = p.into();
        p.x >= self.x()
            && p.x < (self.x() + self.w())
            && p.y >= self.y()
            && p.y < (self.y() + self.h())
    }

    pub fn add_start(self, add: impl Into<Vec2>) -> Self {
        let add = add.into();
        Self {
            start: self.start + add,
            size: self.size - add,
        }
    }

    pub fn sub_size(self, sub: impl Into<Vec2>) -> Self {
        let sub = sub.into();
        Self {
            start: self.start,
            size: self.size - sub,
        }
    }

    pub fn with_margin(self, margin: u16) -> Self {
        self.add_start((margin, margin)).sub_size((margin, margin))
    }

    pub fn split_vertical(self, pos: u16) -> (Self, Self) {
        let up = Self {
            start: self.start,
            size: Vec2 {
                y: pos,
                ..self.size
            },
        };
        let down = Self {
            start: self.start.add_y(pos),
            size: Vec2 {
                y: self.size.y - pos,
                ..self.size
            },
        };

        debug_assert!(self.contains(down.start()));

        (up, down)
    }

    #[inline(always)]
    pub fn end(self) -> Vec2 {
        self.start + self.size
    }

    #[inline(always)]
    pub const fn start(self) -> Vec2 {
        self.start
    }

    #[inline(always)]
    pub const fn size(self) -> Vec2 {
        self.size
    }

    #[inline(always)]
    pub const fn x(self) -> u16 {
        self.start.x
    }

    #[inline(always)]
    pub const fn y(self) -> u16 {
        self.start.y
    }

    #[inline(always)]
    pub const fn w(self) -> u16 {
        self.size.x
    }

    #[inline(always)]
    pub const fn h(self) -> u16 {
        self.size.y
    }
}
