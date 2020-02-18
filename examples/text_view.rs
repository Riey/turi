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
    views::{
        ButtonDecoration,
        ButtonEvent,
        ButtonView,
        Dialog,
        EditView,
        EditViewEvent,
    },
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
    let mut dialog = Dialog::new(EditView::new().map(|v, _s, e| {
        match e {
            EditViewEvent::Edit => {
                log::trace!("edit: {}", v.text());
                false
            }
            EditViewEvent::Submit => {
                log::trace!("submit: {}", v.text());
                true
            }
        }
    }));

    dialog.set_title("TITLE".into());

    dialog.add_button(
        ButtonView::new("Click".into(), ButtonDecoration::Angle),
        |_, s, e| {
            *s += 1;
            log::trace!("btn click count: {}", s);
            match e {
                ButtonEvent::Click => false,
            }
        },
    );

    let mut dialog = dialog.map_e(quit_check);
    let mut count = 0;

    turi::run(&mut count, &mut dialog, &mut printer_guard);
}
