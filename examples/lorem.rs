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
    orientation::Orientation,
    view::{
        ScrollableView,
        View,
    },
    views::ParagraphView,
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
    let mut out = BufWriter::with_capacity(1024 * 1024 * 10, out);

    let backend = CrosstermBackend::new(&mut out, crossterm::terminal::size().unwrap().into());
    let mut guard = CrosstermBackendGuard::new(backend);

    let mut state = false;

    let mut view = ParagraphView::new();

    view.append(include_str!("lorem.txt"));

    let mut view = view
        .consume_event(false)
        .scrollbar(Orientation::Horizontal)
        .or_else_first(|_view, _state, event: Event| {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                }) => Some(true),
                _ => None,
            }
        });

    executor::simple(&mut state, guard.inner(), &mut view, |state, backend| {
        loop {
            match crossterm::event::read().unwrap() {
                Event::Resize(x, y) => {
                    backend.resize((x, y).into());
                    *state = true;
                }
                e => break e,
            }
        }
    });
}
