use crate::view::View;
use crate::update_result::UpdateResult;
use bumpalo::Bump;

pub trait Model {
    type Msg;

    fn update(&mut self, msg: Self::Msg) -> UpdateResult;
    fn view<'a>(&self, b: &'a Bump) -> View<'a, Self::Msg>;
}

