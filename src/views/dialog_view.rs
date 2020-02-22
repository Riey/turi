use crate::{
    event::{
        EventHandler,
        EventLike,
    },
    printer::Printer,
    vec2::Vec2,
    view::View,
    view_wrappers::{
        BoundChecker,
        SizeCacher,
    },
    views::{
        ButtonView,
        LinearView,
    },
};

pub struct DialogView<'a, S, E, M, C: 'a> {
    title:         String,
    content:       SizeCacher<BoundChecker<C>>,
    buttons:       SizeCacher<BoundChecker<LinearView<'a, S, E, M>>>,
    content_focus: bool,
}

impl<'a, S, E, M, C> DialogView<'a, S, E, M, C>
where
    E: EventLike + 'static,
    M: 'static,
{
    pub fn new(content: C) -> Self {
        Self {
            title:         String::new(),
            content:       SizeCacher::new(BoundChecker::new(content)),
            buttons:       SizeCacher::new(BoundChecker::new(LinearView::new())),
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
        btn: ButtonView,
        mut mapper: impl FnMut(&mut S) -> M + 'a,
    ) {
        self.buttons.add_child(
            btn.map_e::<E, _>(|_, _, e| e)
                .map(move |_, state, _| mapper(state)),
        );
    }

    fn tab(&mut self) {
        self.content_focus = !self.content_focus;
    }
}

impl<'a, S, E, M, C> View for DialogView<'a, S, E, M, C>
where
    C: View,
    M: 'static,
{
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
}

impl<'a, S, E, M, C> EventHandler<S, E> for DialogView<'a, S, E, M, C>
where
    C: EventHandler<S, E, Message = M>,
    E: EventLike + 'static,
    M: 'static,
{
    type Message = M;

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
