use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
    KeyModifiers,
};
use simplelog::*;
use std::io::BufWriter;
use turi::{
    backend::{
        CrosstermBackend,
        CrosstermBackendGuard,
    },
    executor,
    state::RedrawState,
    style::Theme,
    view::View,
    views::{
        DialogView,
        EditView,
        EditViewMessage,
    },
};

#[derive(Default, Clone, Copy)]
struct MyState {
    btn_cnt:     u32,
    need_redraw: bool,
}

impl MyState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RedrawState for MyState {
    #[inline]
    fn set_need_redraw(
        &mut self,
        need_redraw: bool,
    ) {
        self.need_redraw = need_redraw;
    }

    #[inline]
    fn is_need_redraw(&self) -> bool {
        self.need_redraw
    }
}

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

    let backend = CrosstermBackend::new(&mut out, crossterm::terminal::size().unwrap().into());
    let mut guard = CrosstermBackendGuard::new(backend);

    let mut state = MyState::new();

    let mut view = DialogView::new(EditView::new().map(|v, _s, m| {
        match m {
            EditViewMessage::Edit => {
                log::trace!("edit: {}", v.text());
                false
            }
            EditViewMessage::Submit => {
                log::trace!("submit: {}", v.text());
                true
            }
        }
    }))
    .title("Title")
    .button("Click", |s: &mut MyState| {
        s.btn_cnt += 1;
        log::trace!("btn click count: {}", s.btn_cnt);
        false
    })
    .or_else_first(|_view, _state, event: Event| {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(true),
            _ => None,
        }
    });

    let theme = Theme::default();

    executor::simple(
        &mut state,
        guard.inner(),
        &theme,
        &mut view,
        |state, backend| {
            loop {
                match crossterm::event::read().unwrap() {
                    Event::Resize(x, y) => {
                        backend.resize((x, y).into());
                        state.set_need_redraw(true);
                    }
                    e => break e,
                }
            }
        },
    );
}
