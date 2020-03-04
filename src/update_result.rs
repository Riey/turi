use std::ops::BitOr;

pub use self::UpdateResult::{
    Exit,
    Ignore,
    Redraw,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpdateResult {
    Redraw,
    Ignore,
    Exit,
}

impl UpdateResult {
    #[inline]
    pub fn is_exit(self) -> bool {
        self == Exit
    }

    #[inline]
    pub fn is_ignore(self) -> bool {
        self == Ignore
    }

    #[inline]
    pub fn is_redraw(self) -> bool {
        self == Redraw
    }
}

impl BitOr for UpdateResult {
    type Output = Self;

    fn bitor(
        self,
        rhs: Self,
    ) -> Self {
        if self.is_redraw() || rhs.is_redraw() {
            Redraw
        } else if self.is_exit() || rhs.is_exit() {
            Exit
        } else if rhs.is_ignore() {
            self
        } else {
            Ignore
        }
    }
}

#[test]
fn or_test() {
    assert_eq!(Redraw | Redraw, Redraw);
    assert_eq!(Redraw | Ignore, Redraw);
    assert_eq!(Ignore | Redraw, Redraw);
    assert_eq!(Ignore | Ignore, Ignore);
}
