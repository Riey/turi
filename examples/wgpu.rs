#![cfg(all(feature = "winit-event", feature = "wgpu-backend"))]

use crossbeam::{
    atomic::AtomicCell,
    channel::bounded,
};
use turi::{
    backend::WgpuBackend,
    executor,
    view::View,
    views::TextView,
};
use winit::{
    event::Event,
    event_loop::ControlFlow,
    window::Window,
};

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

        executor::simple(
            &mut true,
            &mut backend,
            &theme,
            &mut TextView::new("Hello, world!").consume_event(false),
            |redraw, backend| {
                match event_rx.recv().unwrap() {
                    Event::RedrawRequested(_) => {
                        *redraw = true;
                    }
                    _ => {}
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

    event_loop.run(move |e, target, flow| {
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
