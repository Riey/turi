use crossterm::{
    cursor::{
        Hide,
        Show,
    },
    event::{
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode,
        KeyEvent,
        MouseButton,
        MouseEvent,
    },
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use simplelog::*;
use std::io::{
    BufWriter,
    Write,
};
use turi::{
    backend::{
        Backend,
        CrosstermBackend,
    },
    printer::Printer,
    view::View,
    views::{
        SelectView,
        SelectViewEvent,
        SelectViewMessage,
    },
};

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

    enable_raw_mode().unwrap();
    crossterm::execute!(out, Hide, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let mut backend = CrosstermBackend::new(&mut out, crossterm::terminal::size().unwrap().into());

    let mut state = ();

    let mut view = SelectView::with_items(vec![("123".into(), 123), ("456".into(), 456)])
        .map(|view, _state, msg| {
            match msg {
                SelectViewMessage::Select => {
                    log::info!("Selected: {}", view.selected_val());
                    true
                }
                msg => {
                    log::info!("Other event: {:?}", msg);
                    false
                }
            }
        })
        .map_opt_e(|_view, _state, event| {
            match event {
                Event::Key(KeyEvent { code, .. }) => {
                    match code {
                        KeyCode::Enter => Some(SelectViewEvent::Enter),
                        KeyCode::Up => Some(SelectViewEvent::Up),
                        KeyCode::Down => Some(SelectViewEvent::Down),
                        _ => None,
                    }
                }
                Event::Mouse(MouseEvent::Down(MouseButton::Left, _, y, ..)) => {
                    Some(SelectViewEvent::Click(y))
                }
                _ => None,
            }
        });

    view.render(&mut Printer::new(&mut backend));
    backend.flush();

    loop {
        let e = crossterm::event::read().unwrap();

        match e {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => {
                break;
            }
            Event::Resize(x, y) => {
                backend.resize((x, y).into());
            }
            e => {
                match view.on_event(&mut state, e) {
                    Some(msg) => {
                        view.render(&mut Printer::new(&mut backend));
                        backend.flush();
                        if msg {
                            break;
                        }
                    }
                    None => {}
                }
            }
        }
    }

    crossterm::execute!(out, DisableMouseCapture, LeaveAlternateScreen, Show).unwrap();

    disable_raw_mode().unwrap();
}
