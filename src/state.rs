pub trait RedrawState {
    fn set_need_redraw(
        &mut self,
        need_redraw: bool,
    );
    fn is_need_redraw(&self) -> bool;
}

impl RedrawState for bool {
    #[inline(always)]
    fn set_need_redraw(
        &mut self,
        need_redraw: bool,
    ) {
        *self = need_redraw;
    }

    #[inline(always)]
    fn is_need_redraw(&self) -> bool {
        *self
    }
}
