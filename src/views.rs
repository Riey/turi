use crate::view::ViewExt;
use crate::view_wrappers::{BoundChecker, SizeCacher};
use crate::{printer::Printer, style::Style, vec2::Vec2, view::View};
use crossterm::event::{Event, KeyCode, KeyEvent};
use unicode_width::UnicodeWidthStr;

pub struct StyledText {
    spans: Vec<(String, Style)>,
    width: usize,
}

impl StyledText {
    pub fn new() -> Self {
        Self {
            spans: Vec::new(),
            width: 0,
        }
    }

    pub fn styled(text: String, style: Style) -> Self {
        let width = text.width();
        Self {
            spans: vec![(text, style)],
            width,
        }
    }

    pub fn append(&mut self, text: String, style: Style) {
        self.width += text.width();
        self.spans.push((text, style));
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn spans(&self) -> &[(String, Style)] {
        self.spans.as_slice()
    }
}

pub struct TextView {
    text: StyledText,
}

impl TextView {
    pub fn new(text: StyledText) -> Self {
        Self { text }
    }
}

impl<S> View<S> for TextView {
    type Message = ();

    fn desired_size(&self) -> Vec2 {
        Vec2::new(self.text.width() as u16, 1)
    }

    fn layout(&mut self, _size: Vec2) {}

    fn render(&self, printer: &mut Printer) {
        printer.print_styled((0, 0), &self.text);
    }

    fn on_event(&mut self, _state: &mut S, _event: Event) -> Option<Self::Message> {
        None
    }
}

pub struct EditView {
    text: String,
    style: Style,
}

impl EditView {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            style: Style::default(),
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }
}

pub enum EditViewEvent {
    Edit,
    Submit,
}

impl<S> View<S> for EditView {
    type Message = EditViewEvent;

    fn desired_size(&self) -> Vec2 {
        Vec2::new(self.text.width() as u16, 1)
    }

    fn layout(&mut self, _size: Vec2) {}

    fn render(&self, printer: &mut Printer) {
        printer.with_style(self.style, |printer| {
            printer.print((0, 0), &self.text);
        });
    }

    fn on_event(&mut self, _state: &mut S, e: Event) -> Option<Self::Message> {
        match e {
            // TODO: mouse
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => Some(EditViewEvent::Submit),
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => {
                self.text.pop();
                Some(EditViewEvent::Edit)
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                modifiers,
            }) if modifiers.is_empty() => {
                self.text.push(ch);
                Some(EditViewEvent::Edit)
            }
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ButtonDecoration {
    NoDecoration,
    Angle,
}

impl ButtonDecoration {
    #[inline]
    fn decoration(&self, text: &mut String) {
        match self {
            ButtonDecoration::NoDecoration => {}
            ButtonDecoration::Angle => {
                text.insert(0, '<');
                text.push('>');
            }
        }
    }
}

impl Default for ButtonDecoration {
    fn default() -> Self {
        Self::Angle
    }
}

pub struct ButtonView {
    text: String,
    style: Style,
}

impl ButtonView {
    pub fn new(mut text: String, decoration: ButtonDecoration) -> Self {
        decoration.decoration(&mut text);

        Self {
            text,
            style: Style::default(),
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

pub enum ButtonEvent {
    Click,
}

impl<S> View<S> for ButtonView {
    type Message = ButtonEvent;

    fn desired_size(&self) -> Vec2 {
        Vec2::new(self.text.width() as u16, 1)
    }

    fn layout(&mut self, _size: Vec2) {}

    fn render(&self, printer: &mut Printer) {
        printer.with_style(self.style, |printer| {
            printer.print((0, 0), &self.text);
        });
    }

    fn on_event(&mut self, _state: &mut S, e: Event) -> Option<Self::Message> {
        match e {
            // TODO: mouse
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            })
            | Event::Mouse(..) => Some(ButtonEvent::Click),
            _ => None,
        }
    }
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct LinearView<'a, S, M> {
    children: Vec<SizeCacher<BoundChecker<Box<dyn View<S, Message = M> + 'a>>>>,
    orientation: Orientation,
    focus: Option<usize>,
}

impl<'a, S, M> LinearView<'a, S, M> {
    pub fn new() -> Self {
        Self {
            children: Vec::with_capacity(10),
            orientation: Orientation::Horizontal,
            focus: None,
        }
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.set_orientation(orientation);
        self
    }

    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    pub fn add_child(&mut self, v: impl View<S, Message = M> + 'a) {
        self.children
            .push(SizeCacher::new(BoundChecker::new(Box::new(v))));
    }
}

impl<'a, S, M> View<S> for LinearView<'a, S, M> {
    type Message = M;

    fn render(&self, printer: &mut Printer) {
        match self.orientation {
            Orientation::Horizontal => {
                let mut x = 0;
                for child in self.children.iter() {
                    printer.with_bound(printer.bound().add_start((x, 0)), |printer| {
                        child.render(printer)
                    });
                    x += child.prev_size().x;
                }
            }
            Orientation::Vertical => {
                let mut y = 0;
                for child in self.children.iter() {
                    printer.with_bound(printer.bound().add_start((0, y)), |printer| {
                        child.render(printer);
                    });
                    y += child.prev_size().y;
                }
            }
        }
    }

    fn desired_size(&self) -> Vec2 {
        match self.orientation {
            Orientation::Vertical => self
                .children
                .iter()
                .map(|c| c.desired_size())
                .fold(Vec2::new(0, 0), |acc, x| {
                    Vec2::new(acc.x.max(x.x), acc.y + x.y)
                }),
            Orientation::Horizontal => self
                .children
                .iter()
                .map(|c| c.desired_size())
                .fold(Vec2::new(0, 0), |acc, x| {
                    Vec2::new(acc.x + x.x, acc.y.max(x.y))
                }),
        }
    }

    fn layout(&mut self, mut size: Vec2) {
        for child in self.children.iter_mut() {
            let child_size = child.desired_size();
            child.layout(size.min(child_size));

            match self.orientation {
                Orientation::Vertical => {
                    size = size.saturating_sub_y(child_size.y);
                }
                Orientation::Horizontal => {
                    size = size.saturating_sub_x(child_size.x);
                }
            }
        }
    }

    fn on_event(&mut self, state: &mut S, e: Event) -> Option<Self::Message> {
        match e {
            Event::Key(_) => {
                if let Some(focus) = self.focus {
                    self.children[focus].on_event(state, e)
                } else {
                    None
                }
            }
            Event::Mouse(me) => {
                for child in self.children.iter_mut() {
                    if child.contains_cursor(me) {
                        return child.on_event(state, e);
                    }
                }

                None
            }
            Event::Resize(_, _) => None,
        }
    }
}

pub struct Dialog<'a, S, M, C: 'a> {
    title: String,
    content: SizeCacher<BoundChecker<C>>,
    buttons: SizeCacher<BoundChecker<LinearView<'a, S, M>>>,
    content_focus: bool,
}

impl<'a, S, M, C> Dialog<'a, S, M, C>
where
    M: 'static,
{
    pub fn new(content: C) -> Self {
        Self {
            title: String::new(),
            content: SizeCacher::new(BoundChecker::new(content)),
            buttons: SizeCacher::new(BoundChecker::new(LinearView::new())),
            content_focus: true,
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn add_button(
        &mut self,
        btn: ButtonView,
        mapper: impl FnMut(&mut ButtonView, &mut S, ButtonEvent) -> M + 'a,
    ) {
        self.buttons.add_child(btn.map(mapper));
    }

    fn tab(&mut self) {
        self.content_focus = !self.content_focus;
    }
}

impl<'a, S, M, C> View<S> for Dialog<'a, S, M, C>
where
    C: View<S, Message = M>,
    M: 'static,
{
    type Message = M;

    fn render(&self, printer: &mut Printer) {
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

    fn layout(&mut self, size: Vec2) {
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

    fn on_event(&mut self, state: &mut S, e: Event) -> Option<M> {
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
