pub mod backend;
pub mod builder;
pub mod executor;

mod css;
mod element_view;
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
    update_result::{
        Exit,
        Ignore,
        Redraw,
        UpdateResult,
    },
    view::View,
};

pub use ansi_term;
pub use bumpalo;
