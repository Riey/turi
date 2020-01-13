use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use turi::*;

fn quit_check(e: Event) -> Option<bool> {
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
    let mut printer = PrinterGuard::new(
        Printer::new(crossterm::terminal::size().unwrap(), &mut out),
        true,
    );
    let mut dialog = Dialog::new(EditView::new().map(|v, e| {
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
        |_, e| {
            log::trace!("btn click");
            match e {
                ButtonEvent::Click => true,
            }
        },
    );

    let mut dialog = dialog.map_e(quit_check);

    run(&mut dialog, printer.as_printer());
}
