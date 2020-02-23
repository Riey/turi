use crate::{
    event::EventHandler,
    printer::Printer,
    vec2::Vec2,
    view_wrappers::EventMarker,
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

    /// Mark event type for type inference
    #[inline(always)]
    fn mark<E>(self) -> EventMarker<Self, E>
    where
        Self: Sized,
    {
        EventMarker::new(self)
    }
}

pub trait ViewProxy {
    type Inner: View;

    fn get_inner(&self) -> &Self::Inner;
    fn get_inner_mut(&mut self) -> &mut Self::Inner;

    #[inline(always)]
    fn proxy_render(
        &self,
        printer: &mut Printer,
    ) {
        self.get_inner().render(printer);
    }

    #[inline(always)]
    fn proxy_layout(
        &mut self,
        size: Vec2,
    ) {
        self.get_inner_mut().layout(size);
    }

    #[inline(always)]
    fn proxy_desired_size(&self) -> Vec2 {
        self.get_inner().desired_size()
    }
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
        self.proxy_render(printer);
    }

    #[inline(always)]
    fn layout(
        &mut self,
        size: Vec2,
    ) {
        self.proxy_layout(size);
    }

    #[inline(always)]
    fn desired_size(&self) -> Vec2 {
        self.proxy_desired_size()
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

pub trait EventHandledView<S, E>: View + EventHandler<S, E> {}

impl<S, E, V> EventHandledView<S, E> for V where V: View + EventHandler<S, E> {}
