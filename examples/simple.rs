use bumpalo::Bump;
use crossterm::event::Event;
use simplelog::*;
use std::io::BufWriter;
use turi::{
    backend::{
        CrosstermBackend,
        CrosstermBackendGuard,
    },
    builder::{
        attr,
        div,
        text,
    },
    event_filter::EventFilter,
    model::Model,
    style::Theme,
    update_result::{
        Exit,
        Ignore,
        UpdateResult,
    },
    view::View,
};

struct Simple;

impl Model<Event> for Simple {
    type Msg = bool;

    fn update(
        &mut self,
        msg: Self::Msg,
    ) -> UpdateResult {
        if msg {
            Exit
        } else {
            Ignore
        }
    }

    fn view<'a>(
        &self,
        b: &'a Bump,
    ) -> View<'a, Event, Self::Msg> {
        div(b)
            .attr(attr(b).event(EventFilter::ctrl_char(b, 'c', true)).build())
            .children([text("Hello"), text("World!")])
            .build()
    }
}

fn main() {
    WriteLogger::init(
        LevelFilter::Trace,
        ConfigBuilder::new().add_filter_ignore_str("mio").build(),
        std::fs::File::create("turi.log").unwrap(),
    )
    .unwrap();

    let out = turi::util::get_raw_stdout_file();
    let out = BufWriter::with_capacity(1024 * 1024 * 10, out);

    let backend = CrosstermBackend::new(out, crossterm::terminal::size().unwrap().into());
    let mut guard = CrosstermBackendGuard::new(backend);

    let theme = Theme::default();

    log::trace!("Start executor");

    turi::executor::simple(
        guard.inner(),
        &theme,
        &mut Simple,
        |backend, need_redraw| {
            loop {
                match crossterm::event::read().unwrap() {
                    Event::Resize(x, y) => {
                        *need_redraw = true;
                        backend.resize((x, y).into());
                    }
                    e => break e,
                }
            }
        },
    );
}
