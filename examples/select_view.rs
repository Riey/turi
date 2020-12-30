use turi::{
    view::View,
    views::{
        SelectView,
        SelectViewMessage,
    },
};

mod shared;

fn main() {
    self::shared::run(
        false,
        SelectView::with_items(vec![("123".into(), 123), ("456".into(), 456)]).map(
            |view, _state, msg| {
                match msg {
                    SelectViewMessage::Select => {
                        log::info!("Selected: {}", view.selected_val());
                        true
                    }
                    msg => {
                        log::info!("Other event: {:?}", msg);
                        false
                    }
                }
            },
        ),
    );
}
