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
    executor::SimpleExecutor,
    state::RedrawState,
    style::Theme,
    view::View,
};

fn make_view<S, E: Clone + EventLike, V: View<S, E, Message = bool>>(
    view: V
) -> impl View<S, E, Message = bool> {
    view.or_else_first(|_view, _state, event| {
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
    view: V,
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

    let mut view = make_view(view);

    let mut executor = SimpleExecutor::new(state, guard.inner(), theme, view);

    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Resize(x, y) => {
                state.set_need_redraw(true);
                backend.resize((x, y).into());
            }
            e => {
                if executor.on_event(e) {
                    break;
                }
            }
        }
    }
}

#[cfg(all(feature = "wgpu-backend", feature = "winit-event"))]
#[allow(dead_code)]
pub fn wgpu_run<S: RedrawState + 'static, V: View<S, turi::event::WrapWindowEvent, Message = bool> + 'static>(
    state: S,
    view: V,
) {
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
    };

    TermLogger::init(
        LevelFilter::Trace,
        ConfigBuilder::new().add_filter_ignore_str("gfx").build(),
        TerminalMode::Mixed,
    )
    .unwrap();

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    let instance = wgpu::Instance::new(wgpu::BackendBit::all());
    let surface = unsafe { instance.create_surface(&window) };

    let font = wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!(
        "/usr/share/fonts/TTF/D2Coding.ttc"
    ))
    .unwrap();
    let size = window.inner_size();
    let backend = WgpuBackend::new(instance, surface, font, 30.0, (size.width, size.height));
    let theme = Theme::default();
    let view = make_view(view);

    let mut event_state = WrapWindowEventState::new(backend.letter_size());
    let mut executor = SimpleExecutor::new(state, backend, theme, view);

    event_loop.run(move |e, _target, flow| {
        match e {
            Event::RedrawRequested(_) => {
                executor.state.set_need_redraw(true);
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                executor.backend.resize((size.width, size.height));
                executor.state.set_need_redraw(true);
            }
            Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                *flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event, .. } => {
                if executor.on_event(event_state.next_event(event.to_static().unwrap())) {
                    *flow = ControlFlow::Exit;
                } else {
                    *flow = ControlFlow::Poll;
                }
            }
            _ => {}
        }
    });
}

#[allow(dead_code)]
fn main() {}
