#![feature(type_alias_impl_trait)]

use turi::*;

fn main() {
    let mut out = std::io::stdout();
    let mut printer = PrinterGuard::new(Printer::new(Vec2::new(100, 100), &mut out), false);
    let mut view =
        EditView::new().map(|v, e| {
            match e {
                EditViewEvent::Edit => false,
                EditViewEvent::Submit => {
                    println!("\r\ninput: [{}]\r", v.text());
                    true
                },
            }
        });

    run(&mut view, printer.as_printer());
}
