use crate::view::View;

pub trait Model {
    type Msg;
    type View: View;

    fn update(&mut self, msg: Self::Msg);
    fn view(&self) -> Self::View;
}

