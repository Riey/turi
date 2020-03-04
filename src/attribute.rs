use crate::event_filter::EventFilter;

impl<'a, E, M> Clone for Attribute<'a, E, M> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, E, M> Copy for Attribute<'a, E, M> {}

impl<'a, E, M> Default for Attribute<'a, E, M> {
    #[inline]
    fn default() -> Self {
        Self::new(&[], &[])
    }
}

pub struct Attribute<'a, E, M> {
    pub class:  &'a [&'a str],
    pub events: &'a [EventFilter<'a, E, M>],
}

impl<'a, E, M> Attribute<'a, E, M> {
    pub fn new(
        class: &'a [&'a str],
        events: &'a [EventFilter<'a, E, M>],
    ) -> Self {
        Self { class, events }
    }
}
