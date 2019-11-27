use turi::*;
use std::time::Duration;

fn main() {
    let mut out = std::io::stdout();
    let mut printer = Printer::new(Vec2::new(100, 100), &mut out);
    let view = EditView::new();
    let mut inner = Map::new(Source::new(view, TermEventSource(Duration::from_millis(100))), |text| {
        println!("text: {}", text);
        true
    });

    loop {
        //inner.render(&mut printer);
        let ret = inner.on_event(());

        match ret {
            Some(true) => return,
            _ => continue,
        }
    }
}

