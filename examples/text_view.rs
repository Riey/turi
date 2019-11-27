use std::time::Duration;
use turi::*;

fn main() {
    let mut out = std::io::stdout();
    let mut printer = PrinterGuard::new(Printer::new(Vec2::new(100, 100), &mut out), false);
    let view = EditView::new();
    let inner = Map::new(
        Source::new(view, TermEventSource(Duration::from_millis(100))),
        |text| {
            println!("\r\ntext: {}\r", text);
            true
        },
    );

    BasicRunner.run(inner, &mut printer);
}
