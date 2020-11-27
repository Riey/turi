use simplelog::*;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "crossterm-backend", feature = "crossterm-event"))] {
        pub use crossterm_run as run;
    } else if #[cfg(all(feature = "wgpu-backend", feature = "winit-event"))] {
        pub use wgpu_run as run;
    } else {
        compile_error!("Invalid feature");
    }
}

use turi::{
    event::{
        EventLike,
        KeyEventLike,
    },
    executor,
    state::RedrawState,
    style::Theme,
    view::View,
};

fn make_view<S, E: Clone + EventLike, V: View<S, E, Message = bool>>(
    view_factory: impl FnOnce() -> V
) -> impl View<S, E, Message = bool> {
    view_factory().or_else_first(|_view, _state, event| {
        match event.try_key() {
            Some(ke) if ke.try_ctrl_char() == Some('c') => Some(true),
            _ => None,
        }
    })
}

#[cfg(all(feature = "crossterm-backend", feature = "crossterm-event"))]
#[allow(dead_code)]
pub fn crossterm_run<S: RedrawState, V: View<S, crossterm::event::Event, Message = bool>>(
    mut state: S,
    view_factory: impl FnOnce() -> V,
) {
    use std::io::BufWriter;
    use turi::backend::{
        CrosstermBackend,
        CrosstermBackendGuard,
    };
    WriteLogger::init(
        LevelFilter::Trace,
        ConfigBuilder::new().add_filter_ignore_str("mio").build(),
        std::fs::File::create("turi.log").unwrap(),
    )
    .unwrap();

    let out = turi::util::get_raw_stdout_file();
    let out = BufWriter::with_capacity(1024 * 1024 * 10, out);

    let backend = CrosstermBackend::new(out, crossterm::terminal::size().unwrap().into());
    let mut guard = CrosstermBackendGuard::new(backend);

    let theme = Theme::default();

    let mut view = make_view(view_factory);

    executor::simple(
        &mut state,
        guard.inner(),
        &theme,
        &mut view,
        |state, backend| {
            loop {
                match crossterm::event::read().unwrap() {
                    crossterm::event::Event::Resize(x, y) => {
                        state.set_need_redraw(true);
                        backend.resize((x, y).into());
                    }
                    e => break Some(e),
                }
            }
        },
    )
}

#[cfg(all(feature = "wgpu-backend", feature = "winit-event"))]
#[allow(dead_code)]
pub fn wgpu_run<
    S: RedrawState + Send + 'static,
    V: View<S, turi::event::WrapWindowEvent, Message = bool>,
>(
    mut state: S,
    view_factory: impl FnOnce() -> V + Send + 'static,
) {
    use crossbeam::atomic::AtomicCell;
    use turi::{
        backend::WgpuBackend,
        event::WrapWindowEventState,
    };
    use winit::{
        event::{
            Event,
            WindowEvent,
        },
        event_loop::ControlFlow,
        window::Window,
    };

    TermLogger::init(
        LevelFilter::Trace,
        ConfigBuilder::new().build(),
        TerminalMode::Mixed,
    )
    .unwrap();

    static WINDOW: AtomicCell<Option<Window>> = AtomicCell::new(None);

    let (event_tx, event_rx) = crossbeam::channel::unbounded();

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

        let theme = Theme::default();

        let mut view = make_view(view_factory);
        let mut event_state = WrapWindowEventState::new(backend.letter_size());

        executor::simple(
            &mut state,
            &mut backend,
            &theme,
            &mut view,
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
                match event_tx.send(e.to_static().unwrap()) {
                    Ok(_) => {
                        *flow = ControlFlow::Wait;
                    }
                    Err(_) => {
                        *flow = ControlFlow::Exit;
                    }
                }
            }
        }
    });
}

#[allow(dead_code)]
fn main() {}
