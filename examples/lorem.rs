use turi::{
    orientation::Orientation,
    view::View,
    views::{
        FpsView,
        LinearView,
        TextView,
    },
};

mod shared;

fn main() {
    self::shared::run(
        true,
        LinearView::vertical()
            .focus(1)
            .child(FpsView::new().consume_event(false))
            .child(
                TextView::new(include_str!("lorem.txt"))
                    .consume_event(false)
                    .scrollable(Orientation::Horizontal),
            ),
    );
}
