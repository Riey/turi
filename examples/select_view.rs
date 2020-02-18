use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
    KeyModifiers,
};
use simplelog::*;
use std::io::BufWriter;
use turi::{
    printer::PrinterGuard,
    view::View,
    views::SelectView,
};

fn quit_check<S>(
    _s: &mut S,
    e: Event,
) -> Option<bool> {
    match e {
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
        }) => Some(true),
        _ => None,
    }
}

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
    let mut printer_guard = PrinterGuard::new(&mut out, true);

    let mut select_view = SelectView::with_items(vec![("123".into(), 123), ("456".into(), 456)])
        .map(|view, _, _| {
            log::trace!("Input: {}", view.selected_val());
            false
        })
        .map_e(quit_check);

    turi::run(&mut (), &mut select_view, &mut printer_guard);
}
