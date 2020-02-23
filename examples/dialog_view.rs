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
    event::EventHandler,
    executor,
    view::View,
    views::{
        ButtonDecoration,
        ButtonView,
        DialogView,
        EditView,
        EditViewMessage,
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

    let mut state = 0;

    let mut dialog = DialogView::new(EditView::new().mark::<Event>().map(|v, _s, m| {
        match m {
            EditViewMessage::Edit => {
                log::trace!("edit: {}", v.text());
                false
            }
            EditViewMessage::Submit => {
                log::trace!("submit: {}", v.text());
                true
            }
        }
    }));

    dialog.set_title("TITLE".into());
    dialog.add_button(
        ButtonView::new("Click".into(), ButtonDecoration::Angle),
        |s| {
            *s += 1;
            log::trace!("btn click count: {}", s);
            false
        },
    );

    let mut view = dialog
        .mark::<Event>()
        .or_else_first(|_view, _state, event: Event| {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                }) => Some(true),
                _ => None,
            }
        });

    executor::simple(&mut state, guard.inner(), &mut view, |backend| {
        loop {
            match crossterm::event::read().unwrap() {
                Event::Resize(x, y) => backend.resize((x, y).into()),
                e => break e,
            }
        }
    });
}
