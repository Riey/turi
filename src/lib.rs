pub mod backend;
pub mod builder;
pub mod element_view;
pub mod executor;

mod css;
mod event;
mod event_filter;
mod model;
mod printer;
mod rect;
mod update_result;
mod vec2;
mod view;

pub mod util;

pub use self::{
    bumpalo::Bump,
    css::{
        AnsiColor,
        AnsiStyle,
        StyleSheet,
    },
    event::{
        EventLike,
        KeyEventLike,
        MouseEventLike,
    },
    model::Model,
    rect::Rect,
    update_result::{
        Exit,
        Ignore,
        Redraw,
        UpdateResult,
    },
    vec2::Vec2,
    view::View,
};

pub use ansi_term;
pub use bumpalo;
