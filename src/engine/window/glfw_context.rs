use glfw::{ClientApiHint, Glfw, GlfwReceiver, PWindow, WindowEvent, WindowHint, WindowMode};

pub struct GlfwContext {
    context: Glfw,
}

impl GlfwContext {
    pub fn new() -> Self {
        use glfw::fail_on_errors;
        let mut context = glfw::init(fail_on_errors!()).unwrap();

        context.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

        Self { context }
    }

    pub fn poll_events(&mut self) {
        self.context.poll_events();
    }

    pub(super) fn create_window(&mut self,
        width: u32,
        height: u32,
        title: &str,
        mode: WindowMode<'_>
    ) -> (PWindow, GlfwReceiver<(f64, WindowEvent)>) {
        self.context.create_window(width, height, title, mode).expect("Failed to create GLFW window!")
    }
}
