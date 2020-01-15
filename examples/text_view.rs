use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use turi::view::ViewExt;
use turi::{
    printer::PrinterGuard,
    views::{ButtonDecoration, ButtonEvent, ButtonView, Dialog, EditView, EditViewEvent},
};

fn quit_check<S>(_s: &mut S, e: Event) -> Option<bool> {
    match e {
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
        }) => Some(true),
        _ => None,
    }
}

fn main() {
    log_panics::init();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let mut out = std::io::stdout();
    let mut printer_guard = PrinterGuard::new(&mut out, true);
    let mut dialog = Dialog::new(EditView::new().map(|v, _s, e| {
        log::trace!("edit event: {}", v.text());
        match e {
            EditViewEvent::Edit => false,
            EditViewEvent::Submit => {
                println!("\r\ninput: [{}]\r", v.text());
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
                ButtonEvent::Click => true,
            }
        },
    );

    let mut dialog = dialog.map_e(quit_check);
    let mut count = 0;

    turi::run(&mut count, &mut dialog, &mut printer_guard);
}
