use crate::{
    printer::Printer,
    vec2::Vec2,
    view::View,
    view_wrappers::{
        BoundChecker,
        SizeCacher,
    },
    views::{
        ButtonViewEvent,
        ButtonView,
        LinearView,
    },
};
use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
};

pub struct DialogView<'a, S, M, C: 'a> {
    title:         String,
    content:       SizeCacher<BoundChecker<C>>,
    buttons:       SizeCacher<BoundChecker<LinearView<'a, S, M>>>,
    content_focus: bool,
}

impl<'a, S, M, C> DialogView<'a, S, M, C>
where
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
        mapper: impl FnMut(&mut ButtonView, &mut S, ButtonViewEvent) -> M + 'a,
    ) {
        self.buttons.add_child(btn.map(mapper));
    }

    fn tab(&mut self) {
        self.content_focus = !self.content_focus;
    }
}

impl<'a, S, M, C> View<S> for DialogView<'a, S, M, C>
where
    C: View<S, Message = M>,
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
        e: Event,
    ) -> Option<M> {
        match e {
            Event::Key(KeyEvent {
                code: KeyCode::Tab, ..
            }) => {
                self.tab();
                None
            }
            Event::Key(_) => {
                if self.content_focus {
                    self.content.on_event(state, e)
                } else {
                    self.buttons.on_event(state, e)
                }
            }
            Event::Mouse(me) => {
                if self.content.contains_cursor(me) {
                    self.content.on_event(state, e)
                } else if self.buttons.contains_cursor(me) {
                    self.buttons.on_event(state, e)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
