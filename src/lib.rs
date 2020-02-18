pub use crossterm;

#[macro_use]
pub mod macros;

pub mod modifires;
pub mod printer;
pub mod rect;
pub mod style;
pub mod vec2;
pub mod view;
pub mod view_proxys;
pub mod view_wrappers;
pub mod views;

use crate::{
    printer::PrinterGuard,
    view::View,
};

pub fn run<S>(
    state: &mut S,
    view: &mut impl View<S, Message = bool>,
    printer_guard: &mut PrinterGuard,
) {
    let mut printer = printer_guard.make_printer(crossterm::terminal::size().unwrap());
    printer.clear();
    view.layout(printer.bound().size());
    view.render(&mut printer);
    printer.refresh();

    loop {
        let event = crossterm::event::read().unwrap();

        if let crossterm::event::Event::Resize(x, y) = event {
            printer = printer_guard.make_printer((x, y));
        }

        match view.on_event(state, event) {
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
    use crate::{
        style::Style,
        view::*,
        views::*,
    };
    use crossterm::event::{
        Event,
        KeyCode,
        KeyEvent,
        KeyModifiers,
    };

    #[test]
    fn interrupt() {
        let mut view = Dialog::new(
            TextView::new(StyledText::styled("ABC".into(), Style::default())).map(|_, _, _| true),
        )
        .map_e(|_, e| {
            match e {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                }) => Some(true),
                _ => None,
            }
        });

        let ret = view.on_event(
            &mut (),
            Event::Key(KeyEvent {
                code:      KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }),
        );

        assert_eq!(ret, Some(true));
    }
}
