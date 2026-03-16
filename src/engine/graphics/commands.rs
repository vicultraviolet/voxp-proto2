use std::iter;

use wgpu::{Color, CommandEncoder, LoadOp, Operations, RenderPass, RenderPassColorAttachment, RenderPassDescriptor, StoreOp, TextureViewDescriptor, wgt::CommandEncoderDescriptor};

use crate::engine::graphics::{device::Device, pipelines::RenderPipeline, swapchain::Swapchain};

pub struct Commands<'a> {
    device: &'a Device,
    cmd_encoder: Option<CommandEncoder>
}

pub struct GraphicsSession<'a> {
    render_pass: RenderPass<'a>
}

impl<'a> Commands<'a> {
    pub fn begin(device: &'a Device) -> Self {
        let descriptor = CommandEncoderDescriptor {
            label: Some("Encoder")
        };

        Self {
            device,
            cmd_encoder: Some(device.device().create_command_encoder(&descriptor))
        }
    }
}

impl<'a> Drop for Commands<'a> {
    fn drop(&mut self) {
        self.device.queue().submit(iter::once(self.cmd_encoder.take().unwrap().finish()));
    }
}

impl<'a> GraphicsSession<'a> {
    pub fn begin(commands: &'a mut Commands, swapchain: &Swapchain, color: Color) -> Self {
        let output_texture = swapchain.get_current_textre();
        let output_texture_view = output_texture.texture.create_view(&TextureViewDescriptor::default());

        let render_pass_descriptor = RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &output_texture_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(color),
                    store: StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
        };

        Self {
            render_pass: commands.cmd_encoder.as_mut().unwrap().begin_render_pass(&render_pass_descriptor)
        }
    }

    pub fn bind_pipeline(&mut self, pipeline: &RenderPipeline) {
        self.render_pass.set_pipeline(pipeline.pipeline());
    }

    pub fn draw_vertices(&mut self, vertex_count: u32, instance_count: u32) {
        self.render_pass.draw(0..vertex_count, 0..instance_count);
    }
}
