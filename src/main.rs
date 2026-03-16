mod engine;

use std::time::Duration;

use glfw::WindowEvent;
use spin_sleep::sleep;
use wgpu::Color;

use crate::engine::{chrono::timekeeper::Timekeeper, graphics::{commands::{Commands, GraphicsSession}, device::Device, pipelines::RenderPipeline, swapchain::Swapchain}, window::{Window, glfw_context::GlfwContext}};

fn main() {
    env_logger::init();

    let mut glfw_context = GlfwContext::new();
    let window = Window::new(
        &mut glfw_context,
        1280, 720,
        "Voxploration"
    );

    let device = Device::new(&window);

    let mut swapchain = Swapchain::new(&device, &window);

    let pipeline = RenderPipeline::new(&device, swapchain.texture_format());

    let mut timekeeper = Timekeeper::new(144);
    let mut second_accumulator = 0.0f64;

    let mut frame_count = 0u64;

    'running: loop {
        timekeeper.tick();

        glfw_context.poll_events();
        for (_, e) in window.flush_messages() {
            match e {
                WindowEvent::Close => break 'running,
                _ => ()
            }
        }

        frame_count += 1;
        second_accumulator += timekeeper.dt();
        if second_accumulator >= 1.0 {
            println!("FPS: {}", frame_count);

            second_accumulator = 0.0;
            frame_count = 0;
        }

        if window.is_minimized() || !swapchain.acquire_next_image() {
            sleep(Duration::from_millis(33));
            continue;
        }

        {
            let mut commands = Commands::begin(&device);

            {
                let mut session = GraphicsSession::begin(&mut commands, &swapchain, Color::BLACK);
                session.bind_pipeline(&pipeline);
                session.draw_vertices(3, 1);
            }
        }

        swapchain.present();

        timekeeper.pace();
    }
}
