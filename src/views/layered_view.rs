use crate::{
    printer::Printer,
    vec2::Vec2,
    view::View,
};

pub struct LayeredView<S, E, M> {
    layers: Vec<Box<dyn View<S, E, Message = M>>>,
}

impl<S, E, M> LayeredView<S, E, M> {
    pub fn new() -> Self {
        Self {
            layers: Vec::with_capacity(5),
        }
    }

    #[inline]
    pub fn add_layer(
        &mut self,
        layer: impl View<S, E, Message = M> + 'static,
    ) {
        self.layers.push(Box::new(layer));
    }

    #[inline]
    pub fn pop_layer(&mut self) -> Option<Box<dyn View<S, E, Message = M>>> {
        self.layers.pop()
    }
}

impl<S, E, M> View<S, E> for LayeredView<S, E, M> {
    type Message = M;

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        for layer in self.layers.iter().rev() {
            layer.render(printer);
        }
    }

    fn layout(
        &mut self,
        size: Vec2,
    ) {
        for layer in self.layers.iter_mut() {
            layer.layout(size.min(layer.desired_size()));
        }
    }

    fn desired_size(&self) -> Vec2 {
        self.layers
            .iter()
            .map(|layer| layer.desired_size())
            .max()
            .unwrap_or(Vec2::new(0, 0))
    }

    fn on_event(
        &mut self,
        state: &mut S,
        event: E,
    ) -> Option<Self::Message> {
        self.layers.last_mut()?.on_event(state, event)
    }
}
