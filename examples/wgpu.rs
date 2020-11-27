use crossbeam::{
    atomic::AtomicCell,
    channel::bounded,
};
use turi::{
    backend::WgpuBackend,
    executor,
    state::RedrawState,
    view::View,
    views::{
        DialogView,
        EditView,
        EditViewMessage,
    },
};
use winit::{
    event::{
        Event,
        WindowEvent,
    },
    event_loop::ControlFlow,
    window::Window,
};

#[derive(Default, Clone, Copy)]
struct MyState {
    btn_cnt:     u32,
    need_redraw: bool,
}

impl MyState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RedrawState for MyState {
    #[inline]
    fn set_need_redraw(
        &mut self,
        need_redraw: bool,
    ) {
        self.need_redraw = need_redraw;
    }

    #[inline]
    fn is_need_redraw(&self) -> bool {
        self.need_redraw
    }
}

fn main() {
    static WINDOW: AtomicCell<Option<Window>> = AtomicCell::new(None);

    let (event_tx, event_rx) = bounded(1000);

    std::thread::spawn(move || {
        let window = loop {
            if let Some(window) = WINDOW.take() {
                break window;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        };

        let instance = wgpu::Instance::new(wgpu::BackendBit::all());
        let surface = unsafe { instance.create_surface(&window) };

        let font = wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!(
            "/usr/share/fonts/adobe-source-han-sans/SourceHanSansKR-Normal.otf"
        ))
        .unwrap();
        let size = window.inner_size();
        let mut backend =
            WgpuBackend::new(instance, surface, font, 30.0, (size.width, size.height));

        let theme = turi::style::Theme::default();

        let mut event_state = turi::event::WrapWindowEventState::new(backend.letter_size());

        executor::simple(
            &mut MyState::new(),
            &mut backend,
            &theme,
            &mut DialogView::new(EditView::new().map(|v, _s, m| {
                match m {
                    EditViewMessage::Edit => {
                        log::trace!("edit: {}", v.text());
                        false
                    }
                    EditViewMessage::Submit => {
                        log::trace!("submit: {}", v.text());
                        true
                    }
                }
            }))
            .title("Title")
            .button("Click", |s: &mut MyState| {
                s.btn_cnt += 1;
                log::trace!("btn click count: {}", s.btn_cnt);
                false
            }),
            |state, backend| {
                loop {
                    match event_rx.recv().ok()? {
                        Event::RedrawRequested(_) => {
                            state.set_need_redraw(true);
                        }
                        Event::WindowEvent {
                            event: WindowEvent::Resized(size),
                            ..
                        } => {
                            backend.resize((size.width, size.height));
                            state.set_need_redraw(true);
                        }
                        Event::WindowEvent { event, .. } => {
                            break Some(event_state.next_event(event));
                        }
                        _ => {}
                    }
                }
            },
        );
    });

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    WINDOW.store(Some(window));

    event_loop.run(move |e, _target, flow| {
        match e {
            Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                *flow = ControlFlow::Exit;
            }
            _ => {
                event_tx.send(e.to_static().unwrap()).unwrap();
                *flow = ControlFlow::Wait;
            }
        }
    });
}
