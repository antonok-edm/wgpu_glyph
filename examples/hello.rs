use wgpu::winit;
use wgpu_glyph::{GlyphBrushBuilder, Scale, Section};

fn main() -> Result<(), String> {
    env_logger::init();

    // Initialize GPU
    let instance = wgpu::Instance::new();

    let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
        power_preference: wgpu::PowerPreference::HighPerformance,
    });

    let mut device = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits { max_bind_groups: 1 },
    });

    // Open window and create a surface
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .with_resizable(false)
        .build(&events_loop)
        .unwrap();
    let surface = instance.create_surface(&window);

    // Prepare swap chain
    let render_format = wgpu::TextureFormat::Bgra8UnormSrgb;
    let mut size = window
        .get_inner_size()
        .unwrap()
        .to_physical(window.get_hidpi_factor());
    let mut swap_chain = device.create_swap_chain(
        &surface,
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: render_format,
            width: size.width.round() as u32,
            height: size.height.round() as u32,
        },
    );

    // Prepare glyph_brush
    let inconsolata: &[u8] = include_bytes!("Inconsolata-Regular.ttf");
    let mut glyph_brush = GlyphBrushBuilder::using_font_bytes(inconsolata)
        .build(&mut device, render_format);

    // Render loop
    let mut running = true;

    while running {
        // Close window when requested
        events_loop.poll_events(|event| match event {
            winit::Event::WindowEvent {
                event: winit::WindowEvent::CloseRequested,
                ..
            } => running = false,

            winit::Event::WindowEvent {
                event: winit::WindowEvent::Resized(new_size),
                ..
            } => {
                size = new_size.to_physical(window.get_hidpi_factor());

                swap_chain = device.create_swap_chain(
                    &surface,
                    &wgpu::SwapChainDescriptor {
                        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                        format: render_format,
                        width: size.width.round() as u32,
                        height: size.height.round() as u32,
                    },
                );
            }
            _ => {}
        });

        // Get a command encoder for the current frame
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                todo: 0,
            });

        // Get the next frame
        let frame = swap_chain.get_next_texture();

        // Clear frame
        {
            let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color {
                            r: 0.4,
                            g: 0.4,
                            b: 0.4,
                            a: 1.0,
                        },
                    },
                ],
                depth_stencil_attachment: None,
            });
        }

        glyph_brush.queue(Section {
            text: "Hello wgpu_glyph!",
            screen_position: (30.0, 30.0),
            color: [0.0, 0.0, 0.0, 1.0],
            scale: Scale { x: 40.0, y: 40.0 },
            bounds: (size.width as f32, size.height as f32),
            ..Section::default()
        });

        glyph_brush.queue(Section {
            text: "Hello wgpu_glyph!",
            screen_position: (30.0, 90.0),
            color: [1.0, 1.0, 1.0, 1.0],
            scale: Scale { x: 40.0, y: 40.0 },
            bounds: (size.width as f32, size.height as f32),
            ..Section::default()
        });

        // Draw the text!
        glyph_brush.draw_queued(
            &mut device,
            &mut encoder,
            &frame.view,
            size.width.round() as u32,
            size.height.round() as u32,
        )?;

        device.get_queue().submit(&[encoder.finish()]);
    }

    Ok(())
}
