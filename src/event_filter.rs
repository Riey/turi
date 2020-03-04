use crate::event::{
    EventLike,
    KeyEventLike,
    MouseEventLike,
};
use std::marker::PhantomData;

pub struct EventFilter<E, F>
where
    F: Fn(&E) -> bool,
{
    filter:  F,
    _marker: PhantomData<E>,
}

impl<E, F> EventFilter<E, F>
where
    F: Fn(&E) -> bool,
{
    pub fn new(filter: F) -> Self {
        Self {
            filter,
            _marker: PhantomData,
        }
    }

    pub fn check(&self, e: &E) -> bool {
        (self.filter)(e)
    }
}

impl<E> EventFilter<E, fn(&E) -> bool>
where
    E: EventLike,
{
    pub fn click() -> Self {
        Self::new(|e| {
            e.try_mouse()
                .and_then(|me| me.try_left_down())
                .map_or(false, |_| true)
        })
    }
}
