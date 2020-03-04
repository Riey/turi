use crate::event::{
    EventLike,
    KeyEventLike,
    MouseEventLike,
};

impl<'a, E, M> Clone for EventFilter<'a, E, M>
where
    M: Copy,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, E, M> Copy for EventFilter<'a, E, M> where M: Copy {}

pub struct EventFilter<'a, E, M> {
    filter: &'a dyn Fn(&E) -> bool,
    msg:    M,
}

impl<'a, E, M> EventFilter<'a, E, M>
where
    M: Copy,
{
    pub fn new(
        filter: &'a dyn Fn(&E) -> bool,
        msg: M,
    ) -> Self {
        Self { filter, msg }
    }

    pub fn check(
        &self,
        e: &E,
    ) -> Option<M> {
        if (self.filter)(e) {
            Some(self.msg)
        } else {
            None
        }
    }
}

impl<E, M> EventFilter<'static, E, M>
where
    E: EventLike,
    M: Copy,
{
    pub fn empty(msg: M) -> Self {
        Self::new(&|_| false, msg)
    }

    pub fn click(msg: M) -> Self {
        Self::new(
            &|e| {
                e.try_mouse()
                    .and_then(|me| me.try_left_down())
                    .map_or(false, |_| true)
            },
            msg,
        )
    }
}
