use bumpalo::Bump;
use turi::{
    builder::{
        div,
        text,
    },
    model::Model,
    update_result::{
        Ignore,
        UpdateResult,
    },
    view::View,
};

struct Simple;

impl Model for Simple {
    type Msg = ();

    fn update(
        &mut self,
        msg: Self::Msg,
    ) -> UpdateResult {
        Ignore
    }

    fn view<'a>(
        &self,
        b: &'a Bump,
    ) -> View<'a, Self::Msg> {
        div(b).children([text("Hello"), text("World!")]).finish()
    }
}

fn main() {}
