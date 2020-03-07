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
        body,
        class,
        div,
        event,
    },
    Exit,
    Ignore,
    Model,
    StyleSheet,
    UpdateResult,
    View,
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
        div(
            (),
            event(b).ctrl_char('c', true),
            body(b)
                .child(div(class(b).class("hello"), (), "Hello"))
                .child(div((), (), "World!")),
        )
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

    let size = if cfg!(feature = "example-not-use-tty") {
        (40, 50)
    } else {
        crossterm::terminal::size().unwrap()
    };
    let backend = CrosstermBackend::new(out, size.into());
    let mut guard = CrosstermBackendGuard::new(backend);

    let css = StyleSheet::parse(include_str!("../res/simple.css"));

    turi::executor::simple(guard.inner(), &css, &mut Simple, |backend, need_redraw| {
        loop {
            match crossterm::event::read().unwrap() {
                Event::Resize(x, y) => {
                    *need_redraw = true;
                    backend.resize((x, y).into());
                }
                e => break e,
            }
        }
    });
}
