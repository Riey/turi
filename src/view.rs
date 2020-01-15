use crate::{
    printer::Printer,
    vec2::Vec2,
    view_proxys::{Map, MapE},
};
use crossterm::event::Event;

pub trait View {
    type Message;

    fn render(&self, printer: &mut Printer);
    fn layout(&mut self, size: Vec2);
    fn desired_size(&self) -> Vec2;
    fn on_event(&mut self, e: Event) -> Option<Self::Message>;
}

impl<'a, M> View for Box<dyn View<Message = M> + 'a> {
    type Message = M;

    fn render(&self, printer: &mut Printer) {
        (**self).render(printer)
    }
    fn layout(&mut self, size: Vec2) {
        (**self).layout(size)
    }
    fn desired_size(&self) -> Vec2 {
        (**self).desired_size()
    }
    fn on_event(&mut self, e: Event) -> Option<Self::Message> {
        (**self).on_event(e)
    }
}

pub trait ViewExt: View + Sized {
    fn map<F, U>(self, f: F) -> Map<Self, F, U>
    where
        F: FnMut(&mut Self, Self::Message) -> U,
    {
        Map::new(self, f)
    }

    fn map_e<F>(self, f: F) -> MapE<Self, F>
    where
        F: FnMut(Event) -> Option<Self::Message>,
    {
        MapE::new(self, f)
    }
}

impl<V> ViewExt for V where V: View {}

pub trait ViewProxy {
    type Inner: View;
    type Message;

    fn inner_view(&self) -> &Self::Inner;
    fn inner_view_mut(&mut self) -> &mut Self::Inner;

    fn proxy_render(&self, printer: &mut Printer) {
        self.inner_view().render(printer);
    }
    fn proxy_layout(&mut self, size: Vec2) {
        self.inner_view_mut().layout(size);
    }
    fn proxy_desired_size(&self) -> Vec2 {
        self.inner_view().desired_size()
    }
    fn proxy_on_event(&mut self, e: Event) -> Option<Self::Message>;
}

impl<V> View for V
where
    V: ViewProxy,
{
    type Message = V::Message;

    fn render(&self, printer: &mut Printer) {
        self.proxy_render(printer);
    }
    fn layout(&mut self, size: Vec2) {
        self.proxy_layout(size);
    }
    fn desired_size(&self) -> Vec2 {
        self.proxy_desired_size()
    }
    fn on_event(&mut self, e: Event) -> Option<Self::Message> {
        self.proxy_on_event(e)
    }
}
