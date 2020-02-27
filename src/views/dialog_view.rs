use crate::{
    event::{
        EventLike,
        KeyEventLike,
        MouseEventLike,
    },
    printer::Printer,
    state::RedrawState,
    vec2::Vec2,
    view::View,
    view_wrappers::SizeCacher,
    views::{
        ButtonView,
        LinearView,
    },
};

pub struct DialogView<S, E, M, C> {
    title:         String,
    content:       SizeCacher<C>,
    buttons:       LinearView<S, E, M>,
    content_focus: bool,
}

impl<S, E, M, C> DialogView<S, E, M, C>
where
    S: 'static,
    E: EventLike + 'static,
    M: 'static,
{
    pub fn new(content: C) -> Self {
        Self {
            title:         String::new(),
            content:       SizeCacher::new(content),
            buttons:       LinearView::new(),
            content_focus: true,
        }
    }

    pub fn set_title(
        &mut self,
        title: String,
    ) {
        self.title = title;
    }

    pub fn add_button(
        &mut self,
        btn: ButtonView<S, E>,
        mut mapper: impl FnMut(&mut S) -> M + 'static,
    ) {
        self.buttons
            .add_child(btn.map(move |_, state, _| mapper(state)));
    }

    fn tab(&mut self) {
        self.content_focus = !self.content_focus;
    }
}

impl<S, E: EventLike, M, C> View<S, E> for DialogView<S, E, M, C>
where
    S: RedrawState + 'static,
    C: View<S, E, Message = M>,
    E: EventLike + 'static,
    M: 'static,
{
    type Message = M;

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        printer.print_rect();
        printer.print((0, 0), &self.title);
        printer.with_bound(printer.bound().with_margin(1), |printer| {
            let btn_height = 1;
            let bound = printer.bound();
            let (content_bound, btns_bound) =
                printer.bound().split_vertical(bound.h() - btn_height);

            printer.with_bound(content_bound, |printer| {
                self.content.render(printer);
            });

            printer.with_bound(btns_bound, |printer| {
                self.buttons.render(printer);
            });
        });
    }

    fn layout(
        &mut self,
        size: Vec2,
    ) {
        let btn_size = self.buttons.desired_size().min(size);
        let content_size = size.saturating_sub(btn_size);

        self.content.layout(content_size);
        self.buttons.layout(btn_size);
    }

    fn desired_size(&self) -> Vec2 {
        let content = self.content.desired_size();
        let buttons = self.buttons.desired_size();
        Vec2::new(content.x.max(buttons.x), content.y + buttons.y) + Vec2::new(2, 2)
    }

    fn on_event(
        &mut self,
        state: &mut S,
        mut event: E,
    ) -> Option<Self::Message> {
        if let Some(me) = event.try_mouse_mut() {
            let size = self.content.prev_size();
            let is_outline = me.filter_map_pos(|pos| {
                if pos.x == 0 || pos.y == 0 || pos.x > size.x || pos.y > size.y {
                    None
                } else {
                    Some(pos - Vec2::new(1, 1))
                }
            });

            if is_outline {
                return None;
            }

            let is_btn = me.filter_map_pos(|pos| {
                if pos.y == size.y {
                    Some(Vec2::new(pos.x, 0))
                } else {
                    None
                }
            });

            if is_btn {
                self.buttons.on_event(state, event)
            } else {
                self.content.on_event(state, event)
            }
        } else if let Some(ke) = event.try_key() {
            if ke.try_tab() {
                self.tab();
                None
            } else if self.content_focus {
                self.content.on_event(state, event)
            } else {
                self.buttons.on_event(state, event)
            }
        } else {
            None
        }
    }
}
