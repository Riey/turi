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
    style::Theme,
    view::View,
    views::{
        FpsView,
        LinearView,
        ParagraphView,
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
    let mut out = BufWriter::with_capacity(1024 * 1024 * 10, out);

    let backend = CrosstermBackend::new(&mut out, crossterm::terminal::size().unwrap().into());
    let mut guard = CrosstermBackendGuard::new(backend);

    let mut state = false;

    let mut view = ParagraphView::new();

    view.append(include_str!("lorem.txt"));

    let view = view
        .consume_event(false)
        .scrollable(Orientation::Horizontal);

    let mut view = LinearView::new()
        .orientation(Orientation::Vertical)
        .child(FpsView::new().consume_event(false))
        .child(view)
        .or_else_first(|_view, _state, event: Event| {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
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
