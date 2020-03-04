use crate::{
    backend::Backend,
    event_result::EventResult,
    printer::Printer,
    style::Theme,
    view::View,
};

#[cfg(feature = "test-backend")]
use crate::vec2::Vec2;

pub fn simple<S, E, B: Backend, V: View<S, E>>(
    state: &mut S,
    backend: &mut B,
    theme: &Theme,
    view: &mut V,
    is_exit: impl Fn(&S) -> bool,
    mut event_source: impl FnMut(&mut S, &mut B) -> E,
) {
    backend.clear();

    let mut need_redraw = true;

    loop {
        if need_redraw {
            backend.clear();
            view.layout(backend.size());
            view.render(&mut Printer::new(backend, theme));
            backend.flush();
            need_redraw = false
        }
        let e = event_source(state, backend);
        match view.on_event(state, e) {
            EventResult::Consume(redraw) => {
                need_redraw = redraw;
            }
            EventResult::Ignored => continue,
        }

        if is_exit(state) {
            break;
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
