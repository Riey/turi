use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
    MouseButton,
    MouseEvent,
};
use simplelog::*;
use std::io::BufWriter;
use turi::{
    backend::{
        crossterm_run,
        CrosstermBackend,
        CrosstermBackendGuard,
    },
    view::View,
    views::{
        SelectView,
        SelectViewEvent,
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

    let mut state = ();

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
        .map_opt_e(|_view, _state, event| {
            match event {
                Event::Key(KeyEvent { code, .. }) => {
                    match code {
                        KeyCode::Enter => Some(SelectViewEvent::Enter),
                        KeyCode::Up => Some(SelectViewEvent::Up),
                        KeyCode::Down => Some(SelectViewEvent::Down),
                        _ => None,
                    }
                }
                Event::Mouse(MouseEvent::Down(MouseButton::Left, _, y, ..)) => {
                    Some(SelectViewEvent::Click(y))
                }
                _ => None,
            }
        })
        .or_else(|_view, _state, event| {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => Some(true),
                _ => None,
            }
        });

    crossterm_run(&mut state, guard.inner(), &mut view);
}
