#[macro_use]
pub mod macros;

pub mod backend;
pub mod modifires;
pub mod printer;
pub mod rect;
pub mod vec2;
pub mod view;
#[cfg(windows)]
pub mod view_proxys;
pub mod view_wrappers;
pub mod views;

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
        let mut view = DialogView::new(
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
