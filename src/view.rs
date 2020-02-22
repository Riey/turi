use crate::{
    printer::Printer,
    vec2::Vec2,
};

pub trait View {
    fn render(
        &self,
        printer: &mut Printer,
    );
    fn layout(
        &mut self,
        size: Vec2,
    );
    fn desired_size(&self) -> Vec2;
}

pub trait ViewProxy {
    type Inner: View;

    fn get_inner(&self) -> &Self::Inner;
    fn get_inner_mut(&mut self) -> &mut Self::Inner;
}

impl<V, P> View for P
where
    P: ViewProxy<Inner = V>,
    V: View,
{
    #[inline(always)]
    fn render(
        &self,
        printer: &mut Printer,
    ) {
        self.get_inner().render(printer)
    }

    #[inline(always)]
    fn layout(
        &mut self,
        size: Vec2,
    ) {
        self.get_inner_mut().layout(size)
    }

    #[inline(always)]
    fn desired_size(&self) -> Vec2 {
        self.get_inner().desired_size()
    }
}

impl<'a, V> ViewProxy for &'a mut V
where
    V: View,
{
    type Inner = V;

    #[inline(always)]
    fn get_inner(&self) -> &Self::Inner {
        self
    }

    #[inline(always)]
    fn get_inner_mut(&mut self) -> &mut Self::Inner {
        self
    }
}
