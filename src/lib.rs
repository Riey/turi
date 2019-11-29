pub use crossterm;
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton,
    MouseEvent,
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
use derive_more::{Add, From};
use enumflags2::BitFlags;
use std::io::Write;
use std::marker::PhantomData;
use unicode_width::UnicodeWidthStr;

#[derive(Add, Debug, From, Clone, Copy)]
pub struct Vec2<T = usize> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
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

    pub fn as_printer(&mut self) -> &mut Printer<'a> {
        &mut self.printer
    }
}

pub struct Printer<'a> {
    offset: Vec2<u16>,
    bound: Vec2<u16>,
    style: Style,
    out: &'a mut dyn Write,
}

impl<'a> Printer<'a> {
    pub fn new(bound: Vec2<u16>, out: &'a mut dyn Write) -> Self {
        Self {
            offset: Vec2::new(0, 0),
            bound,
            style: Style::default(),
            out,
        }
    }

    pub fn refresh(&mut self) {
        self.out.flush().unwrap();
    }

    pub fn clear(&mut self) {
        queue!(
            self.out,
            SetBackgroundColor(self.style.bg),
            Clear(ClearType::All)
        )
        .unwrap();
    }

    pub fn print(&mut self, start: Vec2<u16>, text: &str) {
        self.print_with_style(start, text, self.style);
    }

    pub fn print_with_style(&mut self, start: Vec2<u16>, text: &str, style: Style) {
        // TODO: cut text when out of bound
        // let width = text.width();
        self.raw_print_with_style(start, text, style);
    }

    fn raw_print_with_style(&mut self, start: Vec2<u16>, text: &str, style: Style) {
        let start = self.offset + start;
        queue!(
            self.out,
            MoveTo(start.x, start.y),
            SetForegroundColor(style.fg),
            SetBackgroundColor(style.bg),
            Output(text)
        )
        .unwrap();
    }

    pub fn print_styled(&mut self, mut start: Vec2<u16>, text: &StyledText) {
        // TODO: cut text when out of bound
        for span in text.spans() {
            let text = &span.0;
            self.raw_print_with_style(start, text, span.1);
            start.x += text.width() as u16;
        }
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
    fn map<F, U>(self, f: F) -> Map<Self, F, U> where F: FnMut(&mut Self, Self::Message) -> U {
        Map {
            inner: self,
            f,
            _marker: PhantomData,
        }
    }
}

impl<V> ViewExt for V where V: View {
}

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

pub trait Sandbox {
    type Message;
    type View: View<Message = Self::Message>;

    fn update(&mut self, msg: Self::Message) -> bool;
    fn view(&self) -> Self::View;
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
        printer.print_styled(Vec2::new(0, 0), &self.text);
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
        printer.print_with_style(Vec2::new(0, 0), &self.text, self.style);
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
        printer.print_with_style(Vec2::new(0, 0), &self.text, self.style);
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

pub struct Map<V, F, U> {
    inner: V,
    f: F,
    _marker: PhantomData<U>,
}

impl<V, F, U> ViewProxy for Map<V, F, U> where V: View, F: FnMut(&mut V, V::Message) -> U {
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
    printer.refresh();

    loop {
        let event = if crossterm::event::poll(std::time::Duration::from_millis(100)).unwrap() {
            crossterm::event::read().unwrap()
        } else {
            continue;
        };

        view.render(printer);
        printer.refresh();

        match view.on_event(event) {
            Some(true) => break,
            _ => {}
        }
    }
}

