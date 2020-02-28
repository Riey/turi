use crate::{
    converters::Map,
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
    views::ButtonView,
};

use ansi_term::Color;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum DialogFocus {
    Content,
    Button(usize),
}

type DialogButton<S, E, M> =
    Map<ButtonView<S, E>, M, Box<dyn FnMut(&mut ButtonView<S, E>, &mut S, ()) -> M>>;

pub struct DialogView<S, E, M, C> {
    title:       String,
    content:     SizeCacher<C>,
    buttons:     Vec<DialogButton<S, E, M>>,
    focus:       DialogFocus,
    focus_color: Color,
}

impl<S, E, M, C> DialogView<S, E, M, C>
where
    S: 'static,
    E: EventLike + 'static,
    M: 'static,
{
    pub fn new(content: C) -> Self {
        Self {
            title:       String::new(),
            content:     SizeCacher::new(content),
            buttons:     Vec::with_capacity(10),
            focus:       DialogFocus::Content,
            focus_color: Color::Yellow,
        }
    }

    #[inline]
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
            .push(btn.map(Box::new(move |_, state, _| mapper(state))));
    }

    #[inline]
    pub fn focus_color(
        mut self,
        focus_color: Color,
    ) -> Self {
        self.focus_color = focus_color;
        self
    }

    #[inline]
    fn tab(&mut self) {
        self.focus = match self.focus {
            DialogFocus::Content if !self.buttons.is_empty() => DialogFocus::Button(0),
            DialogFocus::Content => DialogFocus::Content,
            DialogFocus::Button(n) if n == self.buttons.len() - 1 => DialogFocus::Content,
            DialogFocus::Button(n) => DialogFocus::Button(n + 1),
        };
    }
}

impl<S, E, M, C> View<S, E> for DialogView<S, E, M, C>
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

            let mut x = 0;
            let style = printer.style();

            printer.with_bound(btns_bound, |printer| {
                for (i, btn) in self.buttons.iter().enumerate() {
                    if self.focus == DialogFocus::Button(i) {
                        printer.print_styled(
                            (x, 0),
                            &style.fg(self.focus_color).reverse().paint(btn.text()),
                        );
                    } else {
                        printer.print((x, 0), btn.text());
                    }
                    x += btn.width();
                }
            });
        });
    }

    fn layout(
        &mut self,
        size: Vec2,
    ) {
        // outline
        let size = size.saturating_sub((2, 2).into());
        let content_size = size.saturating_sub_y(1);

        self.content.layout(content_size);
    }

    fn desired_size(&self) -> Vec2 {
        let content = self.content.desired_size();
        let buttons = self.buttons.iter().map(|b| b.width()).sum::<u16>();
        Vec2::new(content.x.max(buttons), content.y + 1) + Vec2::new(2, 2)
    }

    fn on_event(
        &mut self,
        state: &mut S,
        mut event: E,
    ) -> Option<Self::Message> {
        if let Some(me) = event.try_mouse_mut() {
            let size = self.content.prev_size();

            let is_btn = me.filter_map_pos(|pos| {
                if pos.x > 1 && (pos.y - 1) == size.y {
                    Some(Vec2::new(pos.x - 1, 0))
                } else {
                    None
                }
            });

            if is_btn {
                let mut x = me.pos().x;
                for btn in self.buttons.iter_mut() {
                    if btn.width() > x {
                        return btn.on_event(state, event);
                    } else {
                        x -= btn.width();
                    }
                }

                return None;
            }

            let is_content = me.filter_map_pos(|pos| {
                let desired_size = self.content.desired_size();
                if pos.x > 1 && pos.x <= desired_size.x && pos.y > 1 && pos.y <= desired_size.y {
                    Some(pos - Vec2::new(1, 1))
                } else {
                    None
                }
            });

            if is_content {
                self.content.on_event(state, event)
            } else {
                None
            }
        } else if let Some(ke) = event.try_key() {
            if ke.try_tab() {
                self.tab();
                state.set_need_redraw(true);
                None
            } else if self.focus == DialogFocus::Content {
                self.content.on_event(state, event)
            } else if let DialogFocus::Button(x) = &mut self.focus {
                if ke.try_left() {
                    if let Some(new_x) = x.checked_sub(1) {
                        *x = new_x;
                        state.set_need_redraw(true);
                    }
                    None
                } else if ke.try_right() {
                    let new_x = *x + 1;
                    if new_x < self.buttons.len() {
                        *x = new_x;
                        state.set_need_redraw(true);
                    }
                    None
                } else {
                    self.buttons[*x].on_event(state, event)
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
