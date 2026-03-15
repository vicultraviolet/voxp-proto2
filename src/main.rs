use glfw::{WindowEvent, WindowMode, flush_messages};

fn main() {
    use glfw::fail_on_errors;
    let mut glfw_context = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, events) = glfw_context.create_window(
        1280,
        720, 
        "Voxploration", 
        WindowMode::Windowed
    ).unwrap();

    window.set_close_polling(true);

    'running: loop {
        glfw_context.poll_events();
        for (_, e) in flush_messages(&events) {
            match e {
                WindowEvent::Close => break 'running,
                _ => ()
            }
        }
    }
}

