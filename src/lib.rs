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
use std::time::Duration;
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
    fn render(&self, printer: &mut Printer);
    fn layout(&mut self, max: Vec2<u16>) -> Vec2<u16>;
    fn desired_size(&self) -> Vec2<u16>;
}

pub trait ViewProxy {
    type Inner: View;

    fn inner_view(&self) -> &Self::Inner;
    fn inner_view_mut(&mut self) -> &mut Self::Inner;
}

impl<V> View for V
where
    V: ViewProxy,
{
    fn render(&self, printer: &mut Printer) {
        self.inner_view().render(printer);
    }
    fn layout(&mut self, max: Vec2<u16>) -> Vec2<u16> {
        self.inner_view_mut().layout(max)
    }
    fn desired_size(&self) -> Vec2<u16> {
        self.inner_view().desired_size()
    }
}

pub trait EventHandler {
    type Event;
    type Message;

    fn on_event(&mut self, e: Self::Event) -> Option<Self::Message>;
}

pub trait Widget
where
    Self: View + EventHandler,
{
}

impl<T> Widget for T where T: View + EventHandler {}

pub struct Map<U, W, F> {
    inner: W,
    f: F,
    _marker: PhantomData<U>,
}

impl<U, W, F> Map<U, W, F> {
    pub fn new(inner: W, f: F) -> Self {
        Self {
            inner,
            f,
            _marker: PhantomData,
        }
    }
}

impl<U, W, F> EventHandler for Map<U, W, F>
where
    W: Widget,
    F: FnMut(W::Message) -> U,
{
    type Event = W::Event;
    type Message = U;

    fn on_event(&mut self, e: W::Event) -> Option<U> {
        let f = &mut self.f;
        self.inner.on_event(e).map(|m| f(m))
    }
}

impl<U, W, F> ViewProxy for Map<U, W, F>
where
    W: View,
{
    type Inner = W;

    fn inner_view(&self) -> &W {
        &self.inner
    }
    fn inner_view_mut(&mut self) -> &mut W {
        &mut self.inner
    }
}

pub struct Source<W, S> {
    inner: W,
    source: S,
}

impl<W, S> Source<W, S> {
    pub fn new(inner: W, source: S) -> Self {
        Self {
            inner,
            source,
        }
    }
}

impl<W, S> ViewProxy for Source<W, S>
where
    W: View,
{
    type Inner = W;

    fn inner_view(&self) -> &W {
        &self.inner
    }
    fn inner_view_mut(&mut self) -> &mut W {
        &mut self.inner
    }
}

impl<W, S> EventHandler for Source<W, S>
where
    W: Widget,
    S: EventHandler<Message = W::Event>,
{
    type Event = S::Event;
    type Message = W::Message;
    
    fn on_event(&mut self, e: S::Event) -> Option<Self::Message> {
        let se = self.source.on_event(e);
        se.and_then(move |e| self.inner.on_event(e))
    }
}

//impl<'w, E, M, W, SE, S> Widget<'w, SE, M> for Source<E, M, W, SE, S> {}

pub trait Runner {
    type Arg;
    type Result;

    fn run<T>(&mut self, inner: T, printer: &mut PrinterGuard) -> Self::Result
    where
        T: Widget<Event = (), Message = Self::Arg>;
}

pub struct BasicRunner;

impl Runner for BasicRunner {
    type Arg = bool;
    type Result = ();

    fn run<T>(&mut self, mut inner: T, printer: &mut PrinterGuard)
    where
        T: Widget<Event = (), Message = Self::Arg>
    {
        let printer = printer.as_printer();
        printer.clear();
        loop {
            inner.render(printer);
            printer.refresh();
            match inner.on_event(()) {
                Some(true) => return,
                _ => {}
            }
        }
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
    fn desired_size(&self) -> Vec2<u16> {
        Vec2::new(self.text.width() as u16, 1)
    }

    fn layout(&mut self, max: Vec2<u16>) -> Vec2<u16> {
        max
    }

    fn render(&self, printer: &mut Printer) {
        printer.print_styled(Vec2::new(0, 0), &self.text);
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

impl View for EditView {
    fn desired_size(&self) -> Vec2<u16> {
        Vec2::new(self.text.width() as u16, 1)
    }

    fn layout(&mut self, max: Vec2<u16>) -> Vec2<u16> {
        max
    }

    fn render(&self, printer: &mut Printer) {
        printer.print_with_style(Vec2::new(0, 0), &self.text, self.style);
    }
}

impl EventHandler for EditView {
    type Event = Event;
    type Message = String;

    fn on_event(&mut self, e: Event) -> Option<String> {
        match e {
            // TODO: mouse
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => Some(self.text.clone()),
            Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                modifiers,
            }) if modifiers.is_empty() => {
                self.text.push(ch);
                None
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

impl View for ButtonView {
    fn desired_size(&self) -> Vec2<u16> {
        Vec2::new(self.text.width() as u16, 1)
    }

    fn layout(&mut self, max: Vec2<u16>) -> Vec2<u16> {
        max
    }

    fn render(&self, printer: &mut Printer) {
        printer.print_with_style(Vec2::new(0, 0), &self.text, self.style);
    }
}

impl EventHandler for ButtonView {
    type Event = Event;
    type Message = ();

    fn on_event(&mut self, e: Event) -> Option<()> {
        match e {
            // TODO: mouse
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => Some(()),
            _ => None,
        }
    }
}

pub struct TermEventSource(pub Duration);

impl EventHandler for TermEventSource {
    type Event = ();
    type Message = Event;

    fn on_event(&mut self, _: ()) -> Option<Event> {
        match crossterm::event::poll(self.0).unwrap() {
            true => Some(crossterm::event::read().unwrap()),
            false => None,
        }
    }
}
