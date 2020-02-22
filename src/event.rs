use crate::{
    converters::{
        Map,
        MapE,
        MapOptE,
        OrElse,
        OrElseFirst,
    },
    vec2::Vec2,
};

pub trait EventLike {
    fn try_click(&self) -> Option<Vec2>;
    fn try_mouse(&self) -> Option<Vec2>;
    fn try_char(&self) -> Option<char>;
    fn try_enter(&self) -> bool;
    fn try_up(&self) -> bool;
    fn try_down(&self) -> bool;
    fn try_backspace(&self) -> bool;
    fn try_tab(&self) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub struct NoneEvent;

impl EventLike for NoneEvent {
    #[inline(always)]
    fn try_click(&self) -> Option<Vec2> {
        None
    }

    #[inline(always)]
    fn try_mouse(&self) -> Option<Vec2> {
        None
    }

    #[inline(always)]
    fn try_char(&self) -> Option<char> {
        None
    }

    #[inline(always)]
    fn try_enter(&self) -> bool {
        false
    }

    #[inline(always)]
    fn try_up(&self) -> bool {
        false
    }

    #[inline(always)]
    fn try_down(&self) -> bool {
        false
    }

    #[inline(always)]
    fn try_backspace(&self) -> bool {
        false
    }

    #[inline(always)]
    fn try_tab(&self) -> bool {
        false
    }
}

pub trait EventHandler<S, E> {
    type Message;

    fn on_event(
        &mut self,
        state: &mut S,
        event: E,
    ) -> Option<Self::Message>;

    #[inline(always)]
    fn map<U, F>(
        self,
        f: F,
    ) -> Map<Self, U, F>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, Self::Message) -> U,
    {
        Map::new(self, f)
    }

    #[inline(always)]
    fn map_e<NE, F>(
        self,
        f: F,
    ) -> MapE<Self, NE, F>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, E) -> NE,
    {
        MapE::new(self, f)
    }

    #[inline(always)]
    fn map_opt_e<NE, F>(
        self,
        f: F,
    ) -> MapOptE<Self, NE, F>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, NE) -> Option<E>,
    {
        MapOptE::new(self, f)
    }

    #[inline(always)]
    fn or_else<F>(
        self,
        f: F,
    ) -> OrElse<Self, F>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, E) -> Option<Self::Message>,
        E: Clone,
    {
        OrElse::new(self, f)
    }

    #[inline(always)]
    fn or_else_first<F>(
        self,
        f: F,
    ) -> OrElseFirst<Self, F>
    where
        Self: Sized,
        F: FnMut(&mut Self, &mut S, E) -> Option<Self::Message>,
        E: Clone,
    {
        OrElseFirst::new(self, f)
    }
}
