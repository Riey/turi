pub trait EventHandler<S, E> {
    type Message;

    fn on_event(&mut self, state: &mut S, event: E) -> Self::Message;
}

