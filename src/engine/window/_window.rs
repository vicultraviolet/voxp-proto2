use glfw::{FlushedMessages, GlfwReceiver, PWindow, WindowEvent, WindowMode};
use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle};

use crate::engine::window::glfw_context::GlfwContext;

pub struct Window {
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(context: &mut GlfwContext, width: u32, height: u32, title: &str) -> Self {
        let (mut window, events) = context.create_window(width, height, title, WindowMode::Windowed);

        window.set_all_polling(true);

        Self{
            window,
            events
        }
    }

    pub fn flush_messages(&self) -> FlushedMessages<'_, (f64, WindowEvent)> {
        glfw::flush_messages(&self.events)
    }

    pub fn get_width(&self) -> u32 { self.window.get_size().0 as u32 }
    pub fn get_height(&self) -> u32 { self.window.get_size().1 as u32 }

    pub fn is_minimized(&self) -> bool { self.window.is_iconified() }

    pub fn glfw_window(&self) -> &PWindow { &self.window }
}

impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError>
    {
        self.window.window_handle()
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.window.display_handle()
    }
}
