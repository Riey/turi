use super::Backend;
use crate::vec2::Vec2;
use ansi_term::Style;
use futures::executor::block_on;
use wgpu_glyph::{
    ab_glyph::{
        Font,
        FontArc,
    },
    GlyphBrush,
    GlyphBrushBuilder,
    Section,
    Text,
};

pub struct WgpuBackend {
    device:       wgpu::Device,
    queue:        wgpu::Queue,
    staging_belt: wgpu::util::StagingBelt,
    swap_chain:   wgpu::SwapChain,
    glyph_brush:  GlyphBrush<()>,
    letter_size:  (f32, f32),
    window_size:  (u32, u32),
    term_size:    Vec2,
    ansi_style:   Style,
    color:        [f32; 4],
    bg_color:     wgpu::Color,
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

        let render_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let letter_width = font.units_per_em().unwrap_or(std::u16::MAX as f32);
        let letter_height = font_size;

        let term_size = Vec2::new(
            (window_size.0 as f32 / letter_width) as _,
            (window_size.1 as f32 / letter_height) as _,
        );

        Self {
            staging_belt: wgpu::util::StagingBelt::new(1024),
            swap_chain: device.create_swap_chain(&surface, &wgpu::SwapChainDescriptor {
                usage:        wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format:       render_format,
                width:        window_size.0,
                height:       window_size.1,
                present_mode: wgpu::PresentMode::Mailbox,
            }),
            glyph_brush: GlyphBrushBuilder::using_font(font).build(&device, render_format),
            letter_size: (letter_width, letter_height),
            term_size,
            color: [1.0, 0.0, 0.0, 0.0],
            bg_color: wgpu::Color::BLACK,
            ansi_style: Style::default(),
            window_size,
            device,
            queue,
        }
    }
}

impl Backend for WgpuBackend {
    fn clear(&mut self) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("clear"),
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
    }

    fn size(&self) -> crate::vec2::Vec2 {
        self.term_size
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
            .unwrap_or(wgpu::Color::BLACK);
        self.color = [color.a as _, color.r as _, color.g as _, color.b as _];
        self.ansi_style = style;
    }

    fn style(&self) -> Style {
        self.ansi_style
    }

    fn print_at(
        &mut self,
        pos: crate::vec2::Vec2,
        text: &str,
    ) {
        self.glyph_brush.queue(Section {
            screen_position: (
                pos.x as f32 * self.letter_size.0,
                pos.y as f32 * self.letter_size.1,
            ),
            text: vec![Text::new(text)
                .with_color(self.color)
                .with_scale(self.letter_size.1)],
            ..Default::default()
        });
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
        block_on(self.staging_belt.recall());
    }
}
