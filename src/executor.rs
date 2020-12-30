use crate::{
    backend::Backend,
    printer::Printer,
    state::RedrawState,
    style::Theme,
    view::View,
};

use std::marker::PhantomData;

pub struct SimpleExecutor<S: RedrawState, E, B: Backend, V: View<S, E, Message = bool>> {
    pub state:   S,
    pub backend: B,
    pub theme:   Theme,
    pub view:    V,
    _marker:     PhantomData<E>,
}

impl<S: RedrawState, E, B: Backend, V: View<S, E, Message = bool>>
    SimpleExecutor<S, E, B, V>
{
    pub fn new(
        state: S,
        backend: B,
        theme: Theme,
        view: V,
    ) -> Self {
        Self {
            state,
            backend,
            theme,
            view,
            _marker: PhantomData,
        }
    }

    pub fn on_event(
        &mut self,
        e: E,
    ) -> bool {
        if self.state.is_need_redraw() {
            self.backend.clear();
            self.view.layout(self.backend.size());
            self.view
                .render(&mut Printer::new(&mut self.backend, &self.theme));
            self.backend.flush();
            self.state.set_need_redraw(false);
        }

        match self.view.on_event(&mut self.state, e) {
            Some(exit) => exit,
            None => false,
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

#[cfg(feature = "test")]
pub fn test<E, V: View<bool, E>>(
    view: &mut V,
    events: impl IntoIterator<Item = E>,
    size: crate::vec2::Vec2,
    cb: impl FnOnce(&[String]),
) {
    let theme = Theme::default();
    let mut backend = crate::backend::BufferBackend::new(size);
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
