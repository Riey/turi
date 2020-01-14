use derive_more::{Add, AddAssign, From, Sub, SubAssign};

#[derive(
    Add, AddAssign, Sub, SubAssign, Debug, From, Clone, Copy, Ord, PartialOrd, Eq, PartialEq,
)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}

impl Vec2 {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub const fn add_x(self, x: u16) -> Self {
        Self {
            x: self.x + x,
            ..self
        }
    }
    pub const fn add_y(self, y: u16) -> Self {
        Self {
            y: self.y + y,
            ..self
        }
    }
    pub const fn sub_x(self, x: u16) -> Self {
        Self {
            x: self.x - x,
            ..self
        }
    }
    pub const fn sub_y(self, y: u16) -> Self {
        Self {
            y: self.y - y,
            ..self
        }
    }

    pub fn saturating_add(self, other: Self) -> Self {
        Self {
            x: self.x.saturating_add(other.x),
            y: self.y.saturating_add(other.y),
        }
    }

    pub fn saturating_sub(self, other: Self) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }

    pub fn saturating_add_x(self, x: u16) -> Self {
        Self {
            x: self.x.saturating_add(x),
            ..self
        }
    }

    pub fn saturating_add_y(self, y: u16) -> Self {
        Self {
            y: self.y.saturating_add(y),
            ..self
        }
    }

    pub fn saturating_sub_x(self, x: u16) -> Self {
        Self {
            x: self.x.saturating_sub(x),
            ..self
        }
    }

    pub fn saturating_sub_y(self, y: u16) -> Self {
        Self {
            y: self.y.saturating_sub(y),
            ..self
        }
    }

    pub fn max_x(self, x: u16) -> Self {
        Self {
            x: self.x.max(x),
            ..self
        }
    }

    pub fn max_y(self, y: u16) -> Self {
        Self {
            y: self.y.max(y),
            ..self
        }
    }

    pub fn min_x(self, x: u16) -> Self {
        Self {
            x: self.x.min(x),
            ..self
        }
    }

    pub fn min_y(self, y: u16) -> Self {
        Self {
            y: self.y.min(y),
            ..self
        }
    }
}
