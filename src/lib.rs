pub use crossterm;
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, MouseEvent,
};
use crossterm::screen::{EnterAlternateScreen, LeaveAlternateScreen, RawScreen};
use crossterm::style::Color;
use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::{SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    Output,
};
use derive_more::{Add, AddAssign, From, Sub, SubAssign};
use enumflags2::BitFlags;
use std::io::Write;
use std::marker::PhantomData;
use std::mem::replace;
use unicode_width::UnicodeWidthStr;

fn get_pos_from_me(me: MouseEvent) -> Vec2<u16> {
    match me {
        MouseEvent::Up(_, x, y, _)
        | MouseEvent::Down(_, x, y, _)
        | MouseEvent::Drag(_, x, y, _)
        | MouseEvent::ScrollUp(x, y, _)
        | MouseEvent::ScrollDown(x, y, _) => Vec2::new(x, y),
    }
}

#[derive(
    Add, AddAssign, Sub, SubAssign, Debug, From, Clone, Copy, Ord, PartialOrd, Eq, PartialEq,
)]
pub struct Vec2<T = usize> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, From)]
pub struct Rect {
    start: Vec2<u16>,
    size: Vec2<u16>,
}

impl Rect {
    pub fn new(start: impl Into<Vec2<u16>>, size: impl Into<Vec2<u16>>) -> Self {
        Self {
            start: start.into(),
            size: size.into(),
        }
    }

    pub fn contains(self, p: impl Into<Vec2<u16>>) -> bool {
        let p = p.into();
        p.x >= self.x()
            && p.x <= (self.x() + self.w())
            && p.y >= self.y()
            && p.y <= (self.y() + self.h())
    }

    pub fn add_start(self, add: impl Into<Vec2<u16>>) -> Self {
        let add = add.into();
        Self {
            start: self.start + add,
            size: self.size - add,
        }
    }

    pub fn sub_size(self, sub: impl Into<Vec2<u16>>) -> Self {
        let sub = sub.into();
        Self {
            start: self.start,
            size: self.size - sub,
        }
    }

    #[inline(always)]
    pub fn start(self) -> Vec2<u16> {
        self.start
    }

    #[inline(always)]
    pub fn size(self) -> Vec2<u16> {
        self.size
    }

    #[inline(always)]
    pub fn x(self) -> u16 {
        self.start.x
    }

    #[inline(always)]
    pub fn y(self) -> u16 {
        self.start.y
    }

    #[inline(always)]
    pub fn w(self) -> u16 {
        self.size.x
    }

    #[inline(always)]
    pub fn h(self) -> u16 {
        self.size.y
    }
}

#[derive(Clone, Copy, Debug, BitFlags)]
pub enum Modifiers {
    Bold = 0x1,
    Italic = 0x2,
    Reverse = 0x4,
}

#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub modifier: BitFlags<Modifiers>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: BitFlags::empty(),
        }
    }
}

pub struct PrinterGuard<'a> {
    printer: Printer<'a>,
    _raw: RawScreen,
    alternative: bool,
}

impl<'a> Drop for PrinterGuard<'a> {
    fn drop(&mut self) {
        execute!(self.printer.out, DisableMouseCapture).unwrap();
        if self.alternative {
            execute!(self.printer.out, LeaveAlternateScreen).unwrap()
        }
    }
}

impl<'a> PrinterGuard<'a> {
    pub fn new(printer: Printer<'a>, alternative: bool) -> Self {
        execute!(printer.out, EnableMouseCapture,).unwrap();

        if alternative {
            execute!(printer.out, EnterAlternateScreen).unwrap()
        }

        Self {
            printer,
            alternative,
            _raw: RawScreen::into_raw_mode().unwrap(),
        }
    }

    #[inline(always)]
    pub fn as_printer(&mut self) -> &mut Printer<'a> {
        &mut self.printer
    }
}

pub struct Printer<'a> {
    bound: Rect,
    style: Style,
    out: &'a mut dyn Write,
}

impl<'a> Printer<'a> {
    pub fn new(size: Vec2<u16>, out: &'a mut dyn Write) -> Self {
        Self {
            bound: Rect::new((0, 0), size),
            style: Style::default(),
            out,
        }
    }

    pub fn with_bound<T>(&mut self, bound: Rect, f: impl FnOnce(&mut Self) -> T) -> T {
        let old_bound = replace(&mut self.bound, bound);
        let ret = f(self);
        self.bound = old_bound;
        ret
    }

    pub fn with_style<T>(&mut self, style: Style, f: impl FnOnce(&mut Self) -> T) -> T {
        let old_style = replace(&mut self.style, style);
        let ret = f(self);
        self.style = old_style;
        ret
    }

    pub fn refresh(&mut self) {
        self.out.flush().unwrap();
    }

    #[inline(always)]
    pub fn bound(&self) -> Rect {
        self.bound
    }

    pub fn clear(&mut self) {
        queue!(
            self.out,
            SetBackgroundColor(self.style.bg),
            Clear(ClearType::All)
        )
        .unwrap();
    }

    pub fn print(&mut self, start: impl Into<Vec2<u16>>, text: &str) {
        //TODO: check bound
        self.raw_print(start.into(), text);
    }

    fn raw_print(&mut self, start: Vec2<u16>, text: &str) {
        let start = self.bound.start() + start;
        queue!(
            self.out,
            MoveTo(start.x, start.y),
            SetForegroundColor(self.style.fg),
            SetBackgroundColor(self.style.bg),
            Output(text)
        )
        .unwrap();
    }

    pub fn print_styled(&mut self, start: impl Into<Vec2<u16>>, text: &StyledText) {
        let mut start = start.into();
        // TODO: cut text when out of bound
        for span in text.spans() {
            let text = &span.0;
            self.style = span.1;
            self.raw_print(start, text);
            start.x += text.width() as u16;
        }
    }

    pub fn print_vertical_line(&mut self, pos: u16) {
        const VLINE_CHAR: char = '|';

        let pos = self.bound.x() + pos;

        // TODO: check bound
        queue!(
            self.out,
            SetForegroundColor(self.style.fg),
            SetBackgroundColor(self.style.bg),
        )
        .unwrap();

        for i in 0..self.bound.h() {
            queue!(
                self.out,
                MoveTo(pos, self.bound.y() + i),
                Output(VLINE_CHAR),
            )
            .unwrap();
        }
    }

    pub fn print_horizontal_line(&mut self, pos: u16) {
        const HLINE_STR: &str = "―";

        let size = self.bound.w();
        let pos = self.bound.y() + pos;
        let bar = HLINE_STR.repeat(size as usize);

        // TODO: check bound
        queue!(
            self.out,
            SetForegroundColor(self.style.fg),
            SetBackgroundColor(self.style.bg),
            MoveTo(self.bound.x(), pos),
            Output(bar),
        )
        .unwrap();
    }

    pub fn print_rect(&mut self) {
        self.print_horizontal_line(0);
        self.print_horizontal_line(self.bound.h() - 1);
        self.print_vertical_line(0);
        self.print_vertical_line(self.bound.w() - 1);
    }
}

pub trait View {
    type Message;

    fn render(&self, printer: &mut Printer);
    fn layout(&mut self, max: Vec2<u16>) -> Vec2<u16>;
    fn desired_size(&self) -> Vec2<u16>;
    fn on_event(&mut self, e: Event) -> Option<Self::Message>;
}

pub trait ViewExt: View + Sized {
    fn map<F, U>(self, f: F) -> Map<Self, F, U>
    where
        F: FnMut(&mut Self, Self::Message) -> U,
    {
        Map {
            inner: self,
            f,
            _marker: PhantomData,
        }
    }
}

impl<V> ViewExt for V where V: View {}

pub trait ViewProxy {
    type Inner: View;
    type Message;

    fn inner_view(&self) -> &Self::Inner;
    fn inner_view_mut(&mut self) -> &mut Self::Inner;

    fn proxy_render(&self, printer: &mut Printer) {
        self.inner_view().render(printer);
    }
    fn proxy_layout(&mut self, max: Vec2<u16>) -> Vec2<u16> {
        self.inner_view_mut().layout(max)
    }
    fn proxy_desired_size(&self) -> Vec2<u16> {
        self.inner_view().desired_size()
    }
    fn proxy_on_event(&mut self, e: Event) -> Option<Self::Message>;
}

impl<V> View for V
where
    V: ViewProxy,
{
    type Message = V::Message;

    fn render(&self, printer: &mut Printer) {
        self.proxy_render(printer);
    }
    fn layout(&mut self, max: Vec2<u16>) -> Vec2<u16> {
        self.proxy_layout(max)
    }
    fn desired_size(&self) -> Vec2<u16> {
        self.proxy_desired_size()
    }
    fn on_event(&mut self, e: Event) -> Option<Self::Message> {
        self.proxy_on_event(e)
    }
}

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

impl View for TextView {
    type Message = ();

    fn desired_size(&self) -> Vec2<u16> {
        Vec2::new(self.text.width() as u16, 1)
    }

    fn layout(&mut self, max: Vec2<u16>) -> Vec2<u16> {
        max
    }

    fn render(&self, printer: &mut Printer) {
        printer.print_styled((0, 0), &self.text);
    }

    fn on_event(&mut self, _event: Event) -> Option<Self::Message> {
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

impl View for EditView {
    type Message = EditViewEvent;

    fn desired_size(&self) -> Vec2<u16> {
        Vec2::new(self.text.width() as u16, 1)
    }

    fn layout(&mut self, max: Vec2<u16>) -> Vec2<u16> {
        max
    }

    fn render(&self, printer: &mut Printer) {
        printer.with_style(self.style, |printer| {
            printer.print((0, 0), &self.text);
        });
    }

    fn on_event(&mut self, e: Event) -> Option<Self::Message> {
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

impl View for ButtonView {
    type Message = ButtonEvent;

    fn desired_size(&self) -> Vec2<u16> {
        Vec2::new(self.text.width() as u16, 1)
    }

    fn layout(&mut self, max: Vec2<u16>) -> Vec2<u16> {
        max
    }

    fn render(&self, printer: &mut Printer) {
        printer.with_style(self.style, |printer| {
            printer.print((0, 0), &self.text);
        });
    }

    fn on_event(&mut self, e: Event) -> Option<Self::Message> {
        match e {
            // TODO: mouse
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => Some(ButtonEvent::Click),
            _ => None,
        }
    }
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct LinearView<M> {
    children: Vec<(Box<dyn View<Message = M>>, Vec2<u16>)>,
    orientation: Orientation,
    focus: Option<usize>,
}

impl<M> LinearView<M> {
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

    pub fn add_child(&mut self, v: impl View<Message = M> + 'static) {
        self.children.push((Box::new(v), Vec2::new(0, 0)));
    }
}

impl<M> View for LinearView<M> {
    type Message = M;

    fn render(&self, printer: &mut Printer) {
        match self.orientation {
            Orientation::Horizontal => {
                let mut x = 0;
                for child in self.children.iter() {
                    printer.with_bound(printer.bound().add_start((x, 0)), |printer| {
                        child.0.render(printer)
                    });
                    x += child.1.x;
                }
            }
            Orientation::Vertical => {
                let mut y = 0;
                for child in self.children.iter() {
                    printer.with_bound(printer.bound().add_start((0, y)), |printer| {
                        child.0.render(printer);
                    });
                    y += child.1.y;
                }
            }
        }
    }

    fn desired_size(&self) -> Vec2<u16> {
        match self.orientation {
            Orientation::Vertical => {
                self.children
                    .iter()
                    .map(|c| c.0.desired_size())
                    .fold(Vec2::new(0, 0), |acc, x| Vec2::new(acc.x.max(x.x), acc.y + x.y))
            }
            Orientation::Horizontal => {
                self.children
                    .iter()
                    .map(|c| c.0.desired_size())
                    .fold(Vec2::new(0, 0), |acc, x| Vec2::new(acc.x + x.x, acc.y.max(x.y)))
            }
        }
    }

    fn layout(&mut self, max: Vec2<u16>) -> Vec2<u16> {
        let mut actual = Vec2::new(0, 0);
        let mut left = max;
        for (child, size_cache) in self.children.iter_mut() {
            let size = child.layout(left);
            *size_cache = size;
            left -= size;

            match self.orientation {
                Orientation::Vertical => {
                    actual.x = actual.x.max(size.x);
                    actual.y += size.y;
                }
                Orientation::Horizontal => {
                    actual.x += size.x;
                    actual.y = actual.y.max(size.y);
                }
            }
        }
        actual.min(max)
    }

    fn on_event(&mut self, e: Event) -> Option<Self::Message> {
        match e {
            Event::Key(_) => {
                if let Some(focus) = self.focus {
                    self.children[focus].0.on_event(e)
                } else {
                    None
                }
            }
            Event::Mouse(me) => {
                let mut pos = get_pos_from_me(me);

                for (child, size) in self.children.iter_mut() {
                    match self.orientation {
                        Orientation::Vertical => {
                            if pos.y < size.y {
                                return child.on_event(e);
                            }
                            pos.y -= size.y;
                        }
                        Orientation::Horizontal => {
                            if pos.x < size.x {
                                return child.on_event(e);
                            }
                            pos.x -= size.x;
                        }
                    }
                }

                None
            }
            Event::Resize(_, _) => None,
        }
    }
}

pub struct Dialog<M, C> {
    title: String,
    content: C,
    buttons: LinearView<M>,
}

impl<M, C> Dialog<M, C>
where
    M: 'static,
{
    pub fn new(content: C) -> Self {
        Self {
            title: String::new(),
            content,
            buttons: LinearView::new(),
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn add_button(
        &mut self,
        btn: ButtonView,
        mapper: impl FnMut(&mut ButtonView, ButtonEvent) -> M + 'static,
    ) {
        self.buttons.add_child(btn.map(mapper));
    }
}

impl<M, C> View for Dialog<M, C>
where
    C: View,
{
    type Message = M;

    fn render(&self, printer: &mut Printer) {
        printer.print_rect();
        printer.print((0, 0), &self.title);
        printer.with_bound(
            printer.bound().add_start((1, 1)).sub_size((1, 1)),
            |printer| {
                self.content.render(printer);
                let bound = printer.bound();
                printer.with_bound(bound.add_start((0, bound.h() - 1)), |printer| {
                    self.buttons.render(printer);
                });
            },
        );
    }

    fn on_event(&mut self, _e: Event) -> Option<M> {
        unimplemented!()
    }

    fn desired_size(&self) -> Vec2<u16> {
        unimplemented!()
    }

    fn layout(&mut self, _max: Vec2<u16>) -> Vec2<u16> {
        unimplemented!()
    }
}

pub struct Map<V, F, U> {
    inner: V,
    f: F,
    _marker: PhantomData<U>,
}

impl<V, F, U> ViewProxy for Map<V, F, U>
where
    V: View,
    F: FnMut(&mut V, V::Message) -> U,
{
    type Inner = V;
    type Message = U;

    fn inner_view(&self) -> &Self::Inner {
        &self.inner
    }

    fn inner_view_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }

    fn proxy_on_event(&mut self, e: Event) -> Option<U> {
        let msg = self.inner.on_event(e);
        msg.map(|msg| (self.f)(&mut self.inner, msg))
    }
}

pub fn run(view: &mut impl View<Message = bool>, printer: &mut Printer) {
    printer.clear();
    view.render(printer);
    printer.refresh();

    loop {
        let event = if crossterm::event::poll(std::time::Duration::from_millis(100)).unwrap() {
            crossterm::event::read().unwrap()
        } else {
            continue;
        };

        match view.on_event(event) {
            Some(true) => break,
            _ => {}
        }

        printer.clear();
        view.render(printer);
        printer.refresh();
    }
}
