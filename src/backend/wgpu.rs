use super::{
    Backend,
    BufferBackend,
};
use crate::util::calc_term_pos;
use ansi_term::Style;
use futures::{
    executor::{
        block_on,
        LocalPool,
    },
    task::SpawnExt,
};
use wgpu_glyph::{
    ab_glyph::{
        Font,
        FontArc,
    },
    GlyphBrush,
    GlyphBrushBuilder,
    Layout,
    Section,
    Text,
};

pub struct WgpuBackend {
    buffer:       BufferBackend,
    device:       wgpu::Device,
    queue:        wgpu::Queue,
    surface:      wgpu::Surface,
    staging_belt: wgpu::util::StagingBelt,
    swap_chain:   wgpu::SwapChain,
    glyph_brush:  GlyphBrush<()>,
    letter_size:  (f32, f32),
    window_size:  (u32, u32),
    color:        [f32; 4],
    bg_color:     wgpu::Color,
    local_pool:   LocalPool,
}

fn ansi_color_to_gpu_color(c: ansi_term::Color) -> wgpu::Color {
    use ansi_term::Color::{
        Black,
        Blue,
        Cyan,
        Fixed,
        Green,
        Purple,
        Red,
        White,
        Yellow,
        RGB,
    };
    use wgpu::Color;
    match c {
        Black => Color::BLACK,
        White => Color::WHITE,
        Red => Color::RED,
        Green => Color::GREEN,
        Blue => Color::BLUE,
        Cyan => {
            Color {
                r: 0.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }
        }
        Yellow => {
            Color {
                r: 1.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            }
        }
        Purple => {
            Color {
                r: 1.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            }
        }
        RGB(r, g, b) => {
            Color {
                r: r as f64 / 255.0,
                g: g as f64 / 255.0,
                b: b as f64 / 255.0,
                a: 1.0,
            }
        }
        Fixed(..) => todo!(),
    }
}

const RENDER_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

impl WgpuBackend {
    pub fn new(
        instance: wgpu::Instance,
        surface: wgpu::Surface,
        font: FontArc,
        font_size: f32,
        window_size: (u32, u32),
    ) -> Self {
        let (device, queue) = block_on(async {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference:   wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                })
                .await
                .expect("Request adapter");

            adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        features:          wgpu::Features::empty(),
                        limits:            wgpu::Limits::default(),
                        shader_validation: false,
                    },
                    None,
                )
                .await
                .expect("Request device")
        });

        let m_glyph = font.glyph_id('M');
        let m_bound = font.glyph_bounds(&m_glyph.with_scale(font_size));
        let letter_width = m_bound.width();
        let letter_height = m_bound.height();
        let letter_size = (letter_width, letter_height);

        let term_size = calc_term_pos(window_size, letter_size);
        let buffer = BufferBackend::new(term_size);

        Self {
            buffer,
            staging_belt: wgpu::util::StagingBelt::new(1024),
            swap_chain: device.create_swap_chain(&surface, &wgpu::SwapChainDescriptor {
                usage:        wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format:       RENDER_FORMAT,
                width:        window_size.0,
                height:       window_size.1,
                present_mode: wgpu::PresentMode::Mailbox,
            }),
            glyph_brush: GlyphBrushBuilder::using_font(font)
                .initial_cache_size(window_size)
                .build(&device, RENDER_FORMAT),
            letter_size,
            color: [1.0, 1.0, 1.0, 1.0],
            bg_color: wgpu::Color::BLACK,
            window_size,
            device,
            queue,
            surface,
            local_pool: LocalPool::new(),
        }
    }

    pub fn letter_size(&self) -> (f32, f32) {
        self.letter_size
    }

    pub fn resize(
        &mut self,
        window_size: (u32, u32),
    ) {
        self.swap_chain =
            self.device
                .create_swap_chain(&self.surface, &wgpu::SwapChainDescriptor {
                    usage:        wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                    format:       RENDER_FORMAT,
                    width:        window_size.0,
                    height:       window_size.1,
                    present_mode: wgpu::PresentMode::Mailbox,
                });

        self.window_size = window_size;
        self.buffer
            .resize(calc_term_pos(window_size, self.letter_size));
    }
}

impl Backend for WgpuBackend {
    fn clear(&mut self) {
        self.buffer.clear();
    }

    fn size(&self) -> crate::vec2::Vec2 {
        self.buffer.size()
    }

    fn set_style(
        &mut self,
        style: Style,
    ) {
        self.bg_color = style
            .background
            .map(ansi_color_to_gpu_color)
            .unwrap_or(wgpu::Color::BLACK);
        let color = style
            .foreground
            .map(ansi_color_to_gpu_color)
            .unwrap_or(wgpu::Color::WHITE);
        self.color = [color.r as _, color.g as _, color.b as _, color.a as _];
        self.buffer.set_style(style);
    }

    fn style(&self) -> Style {
        self.buffer.style()
    }

    fn print_at(
        &mut self,
        pos: crate::vec2::Vec2,
        text: &str,
    ) {
        self.buffer.print_at(pos, text);
    }

    fn flush(&mut self) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("flush"),
            });

        let frame = self
            .swap_chain
            .get_current_frame()
            .expect("Get next frame")
            .output;

        let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments:        &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment:     &frame.view,
                resolve_target: None,
                ops:            wgpu::Operations {
                    load:  wgpu::LoadOp::Clear(self.bg_color),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        let mut y = 0.0;
        for l in self.buffer.lines().iter() {
            self.glyph_brush.queue(Section {
                screen_position: (0.0, y),
                text: vec![Text::new(l)
                    .with_color(self.color)
                    .with_scale(self.letter_size.1)],
                layout: Layout::default_single_line(),
                ..Default::default()
            });
            y += self.letter_size.1;
        }

        self.glyph_brush
            .draw_queued(
                &self.device,
                &mut self.staging_belt,
                &mut encoder,
                &frame.view,
                self.window_size.0,
                self.window_size.1,
            )
            .expect("draw queued");

        self.staging_belt.finish();
        self.queue.submit(Some(encoder.finish()));
        let spawner = self.local_pool.spawner();
        spawner.spawn(self.staging_belt.recall()).unwrap();
        self.local_pool.run_until_stalled();
    }
}
