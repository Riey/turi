use turi::model::Model;
use turi::view::View;
use turi::builder::{div, text};
use turi::event_result::{
    UpdateResult,
    Ignore,
};
use bumpalo::Bump;

struct Simple;

impl Model for Simple {
    type Msg = ();
    fn update(&mut self, msg: Self::Msg) -> UpdateResult { Ignore }
    fn view<'a>(&self, b: &'a Bump) -> View<'a, Self::Msg> {
        div(b)
            .children([
                text("Hello"),
                text("World!"),
            ])
            .finish()
    }
}

fn main() {
}

