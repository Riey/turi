use crate::{
    backend::Backend,
    event::EventHandler,
    printer::Printer,
    view::View,
};

pub fn simple<S, E, B: Backend, V: View + EventHandler<S, E, Message = bool>>(
    state: &mut S,
    backend: &mut B,
    view: &mut V,
    mut event_source: impl FnMut(&mut B) -> E,
) {
    backend.clear();
    view.layout(backend.size());
    view.render(&mut Printer::new(backend));
    backend.flush();

    loop {
        let e = event_source(backend);
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
