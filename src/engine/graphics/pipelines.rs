use wgpu::{BlendState, ColorTargetState, ColorWrites, FragmentState, FrontFace, MultisampleState, PipelineLayout, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, TextureFormat, VertexState, include_wgsl};

use crate::engine::graphics::device::Device;

pub struct RenderPipeline {
    _layout: PipelineLayout,
    pipeline: wgpu::RenderPipeline,
}

impl RenderPipeline {
    pub fn new(device: &Device, swapchain_format: TextureFormat) -> Self {
        let shader = device.device().create_shader_module(include_wgsl!("shader.wgsl"));

        let layout_descriptor = PipelineLayoutDescriptor {
            label: Some("Triangle Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        };
        let layout = device.device().create_pipeline_layout(&layout_descriptor);

        let descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format: swapchain_format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        };
        let pipeline = device.device().create_render_pipeline(&descriptor);

        Self{
            _layout: layout,
            pipeline
        }
    }

    pub(super) fn pipeline(&self) -> &wgpu::RenderPipeline { &self.pipeline }
}
