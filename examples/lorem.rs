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
        false,
        LinearView::new()
            .orientation(Orientation::Vertical)
            .child(FpsView::new().consume_event(false))
            .child(
                TextView::new(include_str!("lorem.txt"))
                    .consume_event(false)
                    .scrollable(Orientation::Horizontal),
            ),
    );
}
