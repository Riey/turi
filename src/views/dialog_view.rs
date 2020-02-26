use crate::{
    event::{
        EventLike,
    },
    printer::Printer,
    vec2::Vec2,
    view::{
        View,
    },
    view_wrappers::BoundChecker,
    views::{
        ButtonView,
        LinearView,
    },
};

pub struct DialogView<S, E, M, C> {
    title:         String,
    content:       BoundChecker<C>,
    buttons:       BoundChecker<LinearView<S, E, M>>,
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
            content:       BoundChecker::new(content),
            buttons:       BoundChecker::new(LinearView::new()),
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
        self.buttons.inner().add_child(
            btn.map(move |_, state, _| mapper(state)),
        );
    }

    fn tab(&mut self) {
        self.content_focus = !self.content_focus;
    }
}

impl<S, E, M, C> View<S, E> for DialogView<S, E, M, C>
where
    S: 'static,
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
            let btn_height = self.buttons.prev_size().y;
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
        event: E,
    ) -> Option<Self::Message> {
        if event.try_tab() {
            self.tab();
            None
        } else if let Some(pos) = event.try_mouse() {
            if self.content.contains(pos) {
                self.content.on_event(state, event)
            } else if self.buttons.contains(pos) {
                self.buttons.on_event(state, event)
            } else {
                None
            }
        } else {
            if self.content_focus {
                self.content.on_event(state, event)
            } else {
                self.buttons.on_event(state, event)
            }
        }
    }
}
