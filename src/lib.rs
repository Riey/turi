pub use crossterm;

pub mod modifires;
pub mod printer;
pub mod rect;
pub mod style;
pub mod vec2;
pub mod view;
pub mod view_proxys;
pub mod view_wrappers;
pub mod views;

use crate::printer::{Printer, PrinterGuard};
use crate::rect::Rect;
use crate::view::View;

pub fn run(view: &mut impl View<Message = bool>, printer_guard: &mut PrinterGuard) {
    let mut printer = printer_guard.make_printer(crossterm::terminal::size().unwrap());
    printer.clear();
    view.layout(printer.bound().size());
    view.render(&mut printer);
    printer.refresh();

    loop {
        let event = if crossterm::event::poll(std::time::Duration::from_millis(100)).unwrap() {
            crossterm::event::read().unwrap()
        } else {
            continue;
        };

        if let crossterm::event::Event::Resize(x, y) = event {
            printer = printer_guard.make_printer((x, y));
        }

        match view.on_event(event) {
            Some(true) => break,
            _ => {}
        }

        printer.clear();
        view.layout(printer.bound().size());
        view.render(&mut printer);
        printer.refresh();
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{
        KeyModifiers,
        Event,
        KeyEvent,
        KeyCode,
    };
    use crate::views::*;
    use crate::view::*;
    use crate::style::Style;

    #[test]
    fn interrupt() {
        let mut view = Dialog::new(
            TextView::new(StyledText::styled("ABC".into(), Style::default())).map(|_, _| true),
        )
        .map_e(|e| match e {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(true),
            _ => None,
        });

        let ret = view.on_event(Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
        }));

        assert_eq!(ret, Some(true));
    }
}
