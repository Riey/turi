use crate::{
    backend::Backend,
    event::EventHandler,
    printer::Printer,
    view::View,
};
use crate::state::RedrawState;

pub fn simple<S: RedrawState, E, B: Backend, V: View + EventHandler<S, E, Message = bool>>(
    state: &mut S,
    backend: &mut B,
    view: &mut V,
    mut event_source: impl FnMut(&mut B) -> E,
) {
    backend.clear();
    state.set_need_redraw(true);

    loop {
        if state.is_need_redraw() {
            backend.clear();
            view.layout(backend.size());
            view.render(&mut Printer::new(backend));
            backend.flush();
            state.set_need_redraw(false);
        }
        let e = event_source(backend);
        match view.on_event(state, e) {
            Some(exit) => {
                if exit {
                    break;
                }
            }
            None => continue,
        }
    }
}
