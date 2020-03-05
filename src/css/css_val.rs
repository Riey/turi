#[derive(Clone, Copy)]
pub enum CssVal<T: Copy> {
    Val(T),
    Inherit,
}

impl<T: Copy> Default for CssVal<T> {
    fn default() -> Self {
        CssVal::Inherit
    }
}

impl<T: Copy> CssVal<T> {
    pub fn combine(
        self,
        rhs: Self,
    ) -> Self {
        match (self, rhs) {
            (CssVal::Val(v), _) | (_, CssVal::Val(v)) => CssVal::Val(v),
            _ => CssVal::Inherit,
        }
    }

    pub fn and_then(
        self,
        f: impl FnOnce(T) -> Self,
    ) -> Self {
        match self {
            CssVal::Val(val) => f(val),
            CssVal::Inherit => CssVal::Inherit,
        }
    }

    pub fn map<U: Copy>(
        self,
        f: impl FnOnce(T) -> U,
    ) -> CssVal<U> {
        match self {
            CssVal::Val(val) => CssVal::Val(f(val)),
            CssVal::Inherit => CssVal::Inherit,
        }
    }

    pub fn get_or_insert(
        &mut self,
        v: T,
    ) -> &mut T {
        if let CssVal::Inherit = *self {
            *self = CssVal::Val(v);
        }

        match self {
            CssVal::Val(val) => val,
            CssVal::Inherit => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}
