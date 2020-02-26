use crate::{
    printer::Printer,
    vec2::Vec2,
};

pub trait View<S, E> {
    type Message;

    fn render(
        &self,
        printer: &mut Printer,
    );
    fn layout(
        &mut self,
        size: Vec2,
    );
    fn desired_size(&self) -> Vec2;

    fn on_event(
        &mut self,
        state: &mut S,
        event: E,
    ) -> Option<Self::Message>;
}

pub trait ScrollableView<S, E>: View<S, E> {
    fn scroll_vertical_render(
        &self,
        pos: u16,
        printer: &mut Printer,
    );
    fn scroll_horizontal_render(
        &self,
        pos: u16,
        printer: &mut Printer,
    );
    fn scroll_both_render(
        &self,
        pos: Vec2,
        printer: &mut Printer,
    );

    /*
    #[inline(always)]
    fn scrollbar(
        self,
        orientation: Orientation,
    ) -> ScrollView<Self>
    where
        Self: Sized,
    {
        ScrollView::new(self, orientation)
    }
    */
}

