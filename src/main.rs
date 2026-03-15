use std::iter;

use futures::executor::block_on;

use glfw::{WindowEvent, WindowHint, ClientApiHint, WindowMode, flush_messages};
use wgpu::{Backends, BlendState, Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor, DeviceDescriptor, ExperimentalFeatures, Features, FragmentState, FrontFace, Instance, InstanceDescriptor, Limits, LoadOp, MemoryHints, MultisampleState, Operations, PipelineLayoutDescriptor, PolygonMode, PowerPreference, PresentMode, PrimitiveState, PrimitiveTopology, RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor, RequestAdapterOptions, StoreOp, SurfaceConfiguration, TextureUsages, TextureViewDescriptor, Trace, VertexState, include_wgsl};

use crate::timekeeper::Timekeeper;

mod timekeeper;

fn main() {
    env_logger::init();

    use glfw::fail_on_errors;
    let mut glfw_context = glfw::init(fail_on_errors!()).unwrap();

    glfw_context.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

    let (mut window, events) = glfw_context.create_window(
        1280, 720,
        "Voxploration",
        WindowMode::Windowed
    ).unwrap();

    window.set_close_polling(true);

    let instance_descriptor = InstanceDescriptor{
        backends: Backends::VULKAN,
        ..Default::default()
    };
    let instance = Instance::new(&instance_descriptor);

    let surface = instance.create_surface(&window).expect("Failed to create surface!");

    let adapter_options = RequestAdapterOptions{
        power_preference: PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false
    };

    let adapter = instance.request_adapter(&adapter_options);
    let adapter = block_on(adapter).expect("Failed to initialize adapter!");

    let device_descriptor = DeviceDescriptor{
        label: None,
        required_features: Features::empty(),
        experimental_features: ExperimentalFeatures::disabled(),
        required_limits: Limits::default(),
        memory_hints: MemoryHints::Performance,
        trace: Trace::Off
    };

    let device = adapter.request_device(&device_descriptor);
    let (device, queue) = block_on(device).expect("Failed to create device!");

    let surface_caps = surface.get_capabilities(&adapter);

    let surface_format = surface_caps.formats.iter()
        .find(|f| f.is_srgb())
        .cloned()
        .unwrap_or(surface_caps.formats[0]);

    let surface_config = SurfaceConfiguration{
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: window.get_size().0 as u32,
        height: window.get_size().1 as u32,
        present_mode: PresentMode::Mailbox,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: Vec::new(),
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &surface_config);

    let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));

    let pipeline_layout_descriptor = PipelineLayoutDescriptor{
        label: Some("Triangle Pipeline Layout"),
        bind_group_layouts: &[],
        immediate_size: 0
    };
    let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_descriptor);

    let pipeline_descriptor = RenderPipelineDescriptor{
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: VertexState{
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default()
        },
        fragment: Some(FragmentState{
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(ColorTargetState{
                format: surface_config.format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
            })],
            compilation_options: Default::default()
        }),
        primitive: PrimitiveState{
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false
        },
        depth_stencil: None,
        multisample: MultisampleState{
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false
        },
        multiview_mask: None,
        cache: None
    };
    let pipeline = device.create_render_pipeline(&pipeline_descriptor);

    let mut timekeeper = Timekeeper::new(144);
    let mut second_accumulator = 0.0f64;

    let mut frame_count = 0u64;

    'running: loop {
        timekeeper.tick();

        glfw_context.poll_events();
        for (_, e) in flush_messages(&events) {
            match e {
                WindowEvent::Close => break 'running,
                _ => ()
            }
        }

        frame_count += 1;
        second_accumulator += timekeeper.dt();
        if second_accumulator >= 1.0
        {
            println!("FPS: {}", frame_count);

            second_accumulator = 0.0;
            frame_count = 0;
        }

        let output = surface.get_current_texture();
        if output.is_err()
        {
            break;
        }
        let output = output.unwrap();

        let view = output.texture.create_view(&TextureViewDescriptor::default());

        let encoder_descriptor = CommandEncoderDescriptor{
            label: Some("Render")
        };

        let mut encoder = device.create_command_encoder(&encoder_descriptor);

        {
            let render_pass_descriptor = RenderPassDescriptor{
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.02,
                            g: 0.01,
                            b: 0.08,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            };
            let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);

            render_pass.set_pipeline(&pipeline);
            render_pass.draw(0..3, 0..1);
        }

        queue.submit(iter::once(encoder.finish()));

        output.present();

        //timekeeper.pace();
    }
}
