use crate::{
    backend::Backend,
    event::EventHandler,
    printer::Printer,
    view::View,
};

pub fn simple<S, E, V: View + EventHandler<S, E, Message = Option<bool>>>(
    state: &mut S,
    backend: &mut dyn Backend,
    view: &mut V,
    mut event_source: impl FnMut() -> E,
) {
    backend.clear();
    view.layout(backend.size());
    view.render(&mut Printer::new(backend));
    backend.flush();

    loop {
        let e = event_source();
        match view.on_event(state, e) {
            Some(exit) => {
                view.layout(backend.size());
                view.render(&mut Printer::new(backend));
                backend.flush();
                if exit {
                    break;
                }
            }
            None => continue,
        }
    }
}
