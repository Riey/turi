use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}

impl Vec2 {
    pub const fn new(
        x: u16,
        y: u16,
    ) -> Self {
        Self { x, y }
    }

    pub const fn add_x(
        self,
        x: u16,
    ) -> Self {
        Self {
            x: self.x + x,
            ..self
        }
    }

    pub const fn add_y(
        self,
        y: u16,
    ) -> Self {
        Self {
            y: self.y + y,
            ..self
        }
    }

    pub const fn sub_x(
        self,
        x: u16,
    ) -> Self {
        Self {
            x: self.x - x,
            ..self
        }
    }

    pub const fn sub_y(
        self,
        y: u16,
    ) -> Self {
        Self {
            y: self.y - y,
            ..self
        }
    }

    pub fn checked_sub(
        self,
        other: Self,
    ) -> Option<Self> {
        Some(Self {
            x: self.x.checked_sub(other.x)?,
            y: self.y.checked_sub(other.y)?,
        })
    }

    pub fn checked_sub_x(
        self,
        x: u16,
    ) -> Option<Self> {
        Some(Self {
            x: self.x.checked_sub(x)?,
            y: self.y,
        })
    }

    pub fn checked_sub_y(
        self,
        y: u16,
    ) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y.checked_sub(y)?,
        })
    }

    pub fn saturating_add(
        self,
        other: Self,
    ) -> Self {
        Self {
            x: self.x.saturating_add(other.x),
            y: self.y.saturating_add(other.y),
        }
    }

    pub fn saturating_sub(
        self,
        other: Self,
    ) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }

    pub fn saturating_add_x(
        self,
        x: u16,
    ) -> Self {
        Self {
            x: self.x.saturating_add(x),
            ..self
        }
    }

    pub fn saturating_add_y(
        self,
        y: u16,
    ) -> Self {
        Self {
            y: self.y.saturating_add(y),
            ..self
        }
    }

    pub fn saturating_sub_x(
        self,
        x: u16,
    ) -> Self {
        Self {
            x: self.x.saturating_sub(x),
            ..self
        }
    }

    pub fn saturating_sub_y(
        self,
        y: u16,
    ) -> Self {
        Self {
            y: self.y.saturating_sub(y),
            ..self
        }
    }

    pub fn max_x(
        self,
        x: u16,
    ) -> Self {
        Self {
            x: self.x.max(x),
            ..self
        }
    }

    pub fn max_y(
        self,
        y: u16,
    ) -> Self {
        Self {
            y: self.y.max(y),
            ..self
        }
    }

    pub fn min_x(
        self,
        x: u16,
    ) -> Self {
        Self {
            x: self.x.min(x),
            ..self
        }
    }

    pub fn min_y(
        self,
        y: u16,
    ) -> Self {
        Self {
            y: self.y.min(y),
            ..self
        }
    }
}

impl From<(u16, u16)> for Vec2 {
    #[inline]
    fn from((x, y): (u16, u16)) -> Self {
        Vec2 { x, y }
    }
}

impl<T: Into<Vec2>> Add<T> for Vec2 {
    type Output = Self;

    #[inline]
    fn add(
        self,
        rhs: T,
    ) -> Self {
        let rhs = rhs.into();

        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Into<Vec2>> AddAssign<T> for Vec2 {
    #[inline]
    fn add_assign(
        &mut self,
        rhs: T,
    ) {
        let rhs = rhs.into();

        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Into<Vec2>> Sub<T> for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(
        self,
        rhs: T,
    ) -> Self {
        let rhs = rhs.into();

        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Into<Vec2>> SubAssign<T> for Vec2 {
    #[inline]
    fn sub_assign(
        &mut self,
        rhs: T,
    ) {
        let rhs = rhs.into();

        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
