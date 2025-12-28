mod shader;

use crate::shader::compile_and_link_shader;
use gl::{ARRAY_BUFFER, FLOAT, STATIC_DRAW};
use glfw::{Action, Context, GlfwReceiver, Key, fail_on_errors, ffi::glfwGetTime};
use std::ffi::{CString, c_void};

fn main() {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    const WINDOW_WIDTH: u32 = 800;
    const WINDOW_HEIGHT: u32 = 600;
    let (mut window, events) = glfw
        .create_window(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            "OpenGL in Rust",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    let vert_shader_path = "shaders/shader.vert";
    let frag_shader_path = "shaders/shader.frag";
    let shader_program: u32;

    // 5️⃣ Load OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let vertices: [f32; 18] = [
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 1.0,
    ];

    let (mut vbo, mut vao): (u32, u32) = (0, 0);

    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(ARRAY_BUFFER, vbo);
        gl::BufferData(
            ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const c_void,
            STATIC_DRAW,
        );

        //TODO: Fix the unexpected bug
        //vertex array object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        shader_program = compile_and_link_shader(vert_shader_path, frag_shader_path);

        gl::VertexAttribPointer(
            0,
            3,
            FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as i32,
            (3 * std::mem::size_of::<f32>()) as *const c_void,
        );

        gl::EnableVertexAttribArray(1);
    }

    while !window.should_close() {
        process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(shader_program);

            //uniforms
            let time = glfwGetTime() as f32;
            let green_value: f32 = (time.sin() / 2.0) + 0.5;
            //get location of the uniform
            let our_color = CString::new("ourColor").unwrap();
            let vertex_color_location = gl::GetUniformLocation(shader_program, our_color.as_ptr());
            //set uniform
            gl::Uniform4f(vertex_color_location, 0.0, green_value, 0.0, 1.0);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &GlfwReceiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height)
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
