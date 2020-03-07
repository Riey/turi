use std::fmt;

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

impl<'a, E, M: fmt::Debug> fmt::Debug for EventFilter<'a, E, M> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "event(msg: {:?})", self.msg)
    }
}

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
