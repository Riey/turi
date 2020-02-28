use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
};
use simplelog::*;
use std::io::BufWriter;
use turi::{
    backend::{
        CrosstermBackend,
        CrosstermBackendGuard,
    },
    executor,
    view::View,
    views::{
        SelectView,
        SelectViewMessage,
    },
};

fn main() {
    WriteLogger::init(
        LevelFilter::Trace,
        ConfigBuilder::new().add_filter_ignore_str("mio").build(),
        std::fs::File::create("turi.log").unwrap(),
    )
    .unwrap();
    log_panics::init();

    let out = std::io::stdout();
    let out = out.lock();
    let mut out = BufWriter::with_capacity(1024 * 1024, out);

    let backend = CrosstermBackend::new(&mut out, crossterm::terminal::size().unwrap().into());
    let mut guard = CrosstermBackendGuard::new(backend);

    let mut state = false;

    let mut view = SelectView::with_items(vec![("123".into(), 123), ("456".into(), 456)])
        .map(|view, _state, msg| {
            match msg {
                SelectViewMessage::Select => {
                    log::info!("Selected: {}", view.selected_val());
                    true
                }
                msg => {
                    log::info!("Other event: {:?}", msg);
                    false
                }
            }
        })
        .or_else(|_view, _state, event: Event| {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => Some(true),
                _ => None,
            }
        });
    let theme = Theme::default();

    executor::simple(
        &mut state,
        guard.inner(),
        &theme,
        &mut view,
        |state, backend| {
            loop {
                match crossterm::event::read().unwrap() {
                    Event::Resize(x, y) => {
                        backend.resize((x, y).into());
                        *state = true;
                    }
                    e => break e,
                }
            }
        },
    );
}
