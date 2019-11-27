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
use std::io::{self, Write};
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

pub trait EventHandler<E, M> {
    fn on_event(&mut self, e: E) -> Option<M>;
}

pub trait Widget<E, M>
where
    Self: View + EventHandler<E, M>,
{
}

impl<T, E, M> Widget<E, M> for T where T: View + EventHandler<E, M> {}

pub struct Map<E, M, U, W, F> {
    inner: W,
    f: F,
    _marker: PhantomData<(E, M, U)>,
}

impl<E, M, U, W, F> Map<E, M, U, W, F> {
    pub fn new(inner: W, f: F) -> Self {
        Self {
            inner,
            f,
            _marker: PhantomData,
        }
    }
}

impl<E, M, U, W, F> EventHandler<E, U> for Map<E, M, U, W, F>
where
    W: Widget<E, M>,
    F: Fn(M) -> U,
{
    fn on_event(&mut self, e: E) -> Option<U> {
        let f = &mut self.f;
        self.inner.on_event(e).map(|m| f(m))
    }
}

impl<E, M, U, W, F> ViewProxy for Map<E, M, U, W, F>
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

pub struct Source<E, M, W, SE, S> {
    inner: W,
    source: S,
    _marker: PhantomData<(SE, E, M)>,
}

impl<E, M, W, SE, S> Source<E, M, W, SE, S> {
    pub fn new(inner: W, source: S) -> Self {
        Self {
            inner,
            source,
            _marker: PhantomData,
        }
    }
}

impl<E, M, W, SE, S> ViewProxy for Source<E, M, W, SE, S>
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

impl<E, M, W, SE, S> EventHandler<SE, M> for Source<E, M, W, SE, S>
where
    W: Widget<E, M>,
    S: EventHandler<SE, E>,
{
    fn on_event(&mut self, e: SE) -> Option<M> {
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
        T: Widget<(), Self::Arg>;
}

pub struct BasicRunner;

impl Runner for BasicRunner {
    type Arg = bool;
    type Result = ();

    fn run<T>(&mut self, mut inner: T, printer: &mut PrinterGuard)
    where
        T: Widget<(), Self::Arg>,
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

impl EventHandler<Event, String> for EditView {
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

impl EventHandler<Event, ()> for ButtonView {
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

impl EventHandler<(), Event> for TermEventSource {
    fn on_event(&mut self, _: ()) -> Option<Event> {
        match crossterm::event::poll(self.0).unwrap() {
            true => Some(crossterm::event::read().unwrap()),
            false => None,
        }
    }
}
