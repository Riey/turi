use turi::{
    view::View,
    views::{
        LayeredView,
        TextView,
    },
};

mod shared;

fn main() {
    self::shared::run(
        true,
        LayeredView::new()
            .layer(TextView::new("This is second layer").consume_event(false))
            .layer(TextView::new("This is first").consume_event(false)),
    );
}
