use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
    KeyModifiers,
};
use simplelog::*;
use std::io::BufWriter;
use turi::{
    backend::{
        CrosstermBackend,
        CrosstermBackendGuard,
    },
    executor,
    state::RedrawState,
    style::Theme,
    view::View,
};

pub fn run<S: RedrawState>(
    mut state: S,
    view: impl View<S, Event, Message = bool>,
) {
    WriteLogger::init(
        LevelFilter::Trace,
        ConfigBuilder::new().add_filter_ignore_str("mio").build(),
        std::fs::File::create("turi.log").unwrap(),
    )
    .unwrap();

    let out = std::io::stdout();
    let out = out.lock();
    let mut out = BufWriter::with_capacity(1024 * 1024 * 10, out);

    let backend = CrosstermBackend::new(&mut out, crossterm::terminal::size().unwrap().into());
    let mut guard = CrosstermBackendGuard::new(backend);

    let theme = Theme::default();

    let mut view = view.or_else_first(|_view, _state, event: Event| {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(true),
            _ => None,
        }
    });

    executor::simple(
        &mut state,
        guard.inner(),
        &theme,
        &mut view,
        |state, backend| {
            loop {
                match crossterm::event::read().unwrap() {
                    Event::Resize(x, y) => {
                        state.set_need_redraw(true);
                        backend.resize((x, y).into());
                    }
                    e => break e,
                }
            }
        },
    )
}

#[allow(dead_code)]
fn main() {}
