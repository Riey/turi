use turi::*;

fn main() {
    let mut out = std::io::stdout();
    let mut printer = PrinterGuard::new(Printer::new(crossterm::terminal::size().unwrap().into(), &mut out), true);
    let mut dialog =
        Dialog::new(EditView::new().map(|v, e| {
            match e {
                EditViewEvent::Edit => false,
                EditViewEvent::Submit => {
                    println!("\r\ninput: [{}]\r", v.text());
                    true
                },
            }
        }));

    dialog.set_title("TITLE".into());

    dialog.add_button(ButtonView::new("Click".into(), ButtonDecoration::Angle), |_, e| {
        match e {
            ButtonEvent::Click => true,
        }
    });

    run(&mut dialog, printer.as_printer());
}

