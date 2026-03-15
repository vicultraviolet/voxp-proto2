use glfw::{WindowEvent, WindowMode, flush_messages};

use crate::timekeeper::Timekeeper;

mod timekeeper;

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

        timekeeper.pace();
    }
}

