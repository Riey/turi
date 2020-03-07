use crate::{
    backend::Backend,
    css::StyleSheet,
    event::EventLike,
    model::Model,
    printer::Printer,
    update_result::UpdateResult,
};

use bumpalo::Bump;

use nohash_hasher::IntMap;

#[cfg(feature = "test-backend")]
use crate::vec2::Vec2;

pub fn simple<E: EventLike + Copy, B: Backend, M: Model<E>>(
    backend: &mut B,
    css: &StyleSheet,
    model: &mut M,
    mut event_source: impl FnMut(&mut B, &mut bool) -> E,
) where
    M::Msg: Copy,
{
    let mut cache = IntMap::default();
    let mut bump = Bump::with_capacity(1024 * 1024);
    backend.clear();

    let mut view = model.view(&bump);
    let mut need_redraw = true;

    loop {
        if need_redraw {
            backend.clear();
            view.render(css, &mut Printer::new(backend), &mut cache);
            backend.flush();
            need_redraw = false
        }
        let e = event_source(backend, &mut need_redraw);
        match view.on_event(e) {
            Some(msg) => {
                match model.update(msg) {
                    UpdateResult::Redraw => {
                        need_redraw = true;
                        bump.reset();
                        view = model.view(&bump);
                    }
                    UpdateResult::Ignore => continue,
                    UpdateResult::Exit => return,
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
