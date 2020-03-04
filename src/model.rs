use crate::{
    update_result::UpdateResult,
    view::View,
};
use bumpalo::Bump;

pub trait Model<E> {
    type Msg;

    fn update(
        &mut self,
        msg: Self::Msg,
    ) -> UpdateResult;
    fn view<'a>(
        &self,
        b: &'a Bump,
    ) -> View<'a, E, Self::Msg>;
}
