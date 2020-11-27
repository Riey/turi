use turi::{
    state::RedrawState,
    view::View,
    views::{
        DialogView,
        EditView,
        EditViewMessage,
    },
};

mod shared;

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
    self::shared::run(MyState::new(), || {
        DialogView::new(EditView::new().map(|v, _s, m| {
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
    });
}
