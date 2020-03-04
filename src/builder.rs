use crate::view::View;
use bumpalo::{
    collections::Vec,
    Bump,
};

pub struct DivBuilder<'a, M> {
    children: Vec<'a, View<'a, M>>,
}

impl<'a, M> DivBuilder<'a, M> {
    pub fn new(b: &'a Bump) -> Self {
        Self {
            children: Vec::new_in(b),
        }
    }

    #[inline]
    pub fn child(
        mut self,
        child: View<'a, M>,
    ) -> Self {
        self.children.push(child);
        self
    }

    #[inline]
    pub fn children(
        mut self,
        children: impl AsRef<[View<'a, M>]>,
    ) -> Self {
        self.children.extend_from_slice(children.as_ref());
        self
    }

    #[inline]
    pub fn finish(self) -> View<'a, M> {
        View::Div(self.children.into_bump_slice())
    }
}

pub fn div<M>(b: &Bump) -> DivBuilder<M> {
    DivBuilder::new(b)
}

pub fn text<'a, M>(text: &'a str) -> View<'a, M> {
    View::Text(text)
}

