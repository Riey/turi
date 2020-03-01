use crate::{
    backend::Backend,
    printer::Printer,
    state::RedrawState,
    style::Theme,
    vec2::Vec2,
    view::View,
};

pub fn simple<S: RedrawState, E, B: Backend, V: View<S, E, Message = bool>>(
    state: &mut S,
    backend: &mut B,
    theme: &Theme,
    view: &mut V,
    mut event_source: impl FnMut(&mut S, &mut B) -> E,
) {
    backend.clear();
    state.set_need_redraw(true);

    loop {
        if state.is_need_redraw() {
            backend.clear();
            view.layout(backend.size());
            view.render(&mut Printer::new(backend, theme));
            backend.flush();
            state.set_need_redraw(false);
        }
        let e = event_source(state, backend);
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

#[cfg(feature = "bench")]
pub fn bench<B: Backend, E, V: View<bool, E>>(
    backend: &mut B,
    view: &mut V,
    events: impl IntoIterator<Item = E>,
) {
    let theme = Theme::default();
    let mut printer = Printer::new(backend, &theme);

    let mut need_redraw = true;

    for event in events {
        if need_redraw {
            view.layout(printer.bound().size());
            view.render(&mut printer);
            need_redraw = false;
        }

        view.on_event(&mut need_redraw, event);
    }
}

#[cfg(feature = "test-backend")]
pub fn test<E, V: View<bool, E>>(
    view: &mut V,
    events: impl IntoIterator<Item = E>,
    size: Vec2,
    cb: impl FnOnce(&[String]),
) {
    let theme = Theme::default();
    let mut backend = crate::backend::TestBackend::new(size);
    let mut printer = Printer::new(&mut backend, &theme);

    let mut state = false;

    view.layout(size);
    view.render(&mut printer);

    for event in events {
        view.on_event(&mut state, event);
    }

    if state {
        printer.clear();
        view.layout(size);
        view.render(&mut printer);
    }

    cb(backend.lines());
}
