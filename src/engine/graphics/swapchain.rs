use wgpu::{PresentMode, Surface, SurfaceConfiguration, SurfaceTexture, TextureFormat, TextureUsages};

use crate::engine::{graphics::device::Device, window::Window};

pub struct Swapchain<'a> {
    surface: Surface<'a>,
    config: SurfaceConfiguration,
    current_texture: Option<SurfaceTexture>
}

impl<'a> Swapchain<'a> {
    pub fn new(device: &Device, window: &'a Window) -> Self {
        let surface = device.instance()
            .create_surface(window.glfw_window())
            .expect("Failed to create window surface!");

        let surface_caps = surface.get_capabilities(device.adapter());

        let format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .cloned()
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: format,
            width: window.get_width(),
            height: window.get_height(),
            present_mode: PresentMode::Mailbox,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: Vec::new(),
            desired_maximum_frame_latency: 2,
        };

        surface.configure(device.device(), &config);

        Self{
            surface,
            config,
            current_texture: None
        }
    }

    pub fn acquire_next_image(&mut self) -> bool {
        let output_texture = self.surface.get_current_texture();
        if output_texture.is_err() {
            return false;
        }

        self.current_texture = Some(output_texture.unwrap());

        true
    }

    pub fn present(&mut self) {
        if let Some(current_texture) = self.current_texture.take()
        {
            current_texture.present();
        }
    }

    pub fn texture_format(&self) -> TextureFormat { self.config.format }

    pub(super) fn current_texture(&self) -> Option<&SurfaceTexture> { self.current_texture.as_ref() }
    pub(super) fn get_current_textre(&self) -> &SurfaceTexture { self.current_texture.as_ref().expect("Swapchain image not acquired!") }
}
