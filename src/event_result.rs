use std::ops::BitOr;

pub const REDRAW: EventResult = EventResult::Consume(true);
pub const NODRAW: EventResult = EventResult::Consume(false);
pub const IGNORE: EventResult = EventResult::Ignored;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventResult {
    Consume(bool),
    Ignored,
}

impl EventResult {
    #[inline]
    pub fn is_consume(self) -> bool {
        !self.is_ignored()
    }

    #[inline]
    pub fn is_ignored(self) -> bool {
        self == IGNORE
    }

    #[inline]
    pub fn is_redraw(self) -> bool {
        self == REDRAW
    }

    #[inline]
    pub fn is_nodraw(self) -> bool {
        self == NODRAW
    }
}

impl BitOr for EventResult {
    type Output = Self;

    fn bitor(
        self,
        rhs: Self,
    ) -> Self {
        if self.is_redraw() || rhs.is_redraw() {
            REDRAW
        } else if rhs.is_ignored() {
            self
        } else {
            rhs
        }
    }
}

#[test]
fn or_test() {
    assert_eq!(REDRAW | REDRAW, REDRAW);
    assert_eq!(REDRAW | NODRAW, REDRAW);
    assert_eq!(REDRAW | IGNORE, REDRAW);
    assert_eq!(NODRAW | REDRAW, REDRAW);
    assert_eq!(NODRAW | NODRAW, NODRAW);
    assert_eq!(NODRAW | IGNORE, NODRAW);
    assert_eq!(IGNORE | REDRAW, REDRAW);
    assert_eq!(IGNORE | NODRAW, NODRAW);
    assert_eq!(IGNORE | IGNORE, IGNORE);
}
