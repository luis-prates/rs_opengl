extern crate glfw;

use gl::types::{GLfloat, GLsizeiptr};

use self::glfw::{Action, Context, Key};

extern crate gl;
use self::gl::types::*;

use std::ffi::CString;
use std::{mem, ptr, str};
use std::os::raw::c_void;
use std::sync::mpsc::Receiver;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAG_SHADER_SOURCE: &str = r#"
	#version 330 core
	out vec4 FragColor;

	void main() {
		FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
	} 
"#;

pub fn main_1_2_4() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "LearnOpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

	let (shader_program, mut vbos, mut vaos) = unsafe {
		let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
		let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
		gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
		gl::CompileShader(vertex_shader);

		let mut success = gl::FALSE as GLint;
		let mut info_log = Vec::with_capacity(512);
		info_log.set_len(512 - 1);
		gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
		if success != gl::TRUE as GLint {
			gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
			println!("ERROR::SHADER::VERTEX::COMPILATIOM_FAILED\n{}", str::from_utf8(&info_log).unwrap());
		};

		let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
		let c_str_frag = CString::new(FRAG_SHADER_SOURCE.as_bytes()).unwrap();
		gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
		gl::CompileShader(fragment_shader);
		gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
		if success != gl::TRUE as GLint {
			gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
			println!("ERROR::SHADER::FRAG::COMPILATIOM_FAILED\n{}", str::from_utf8(&info_log).unwrap());
		};

		let shader_program = gl::CreateProgram();
		gl::AttachShader(shader_program, vertex_shader);
		gl::AttachShader(shader_program, fragment_shader);
		gl::LinkProgram(shader_program);
		gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
		if success != gl::TRUE as GLint {
			gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
			println!("ERROR::SHADER::PROGRAM::LINKING::COMPILATIOM_FAILED\n{}", str::from_utf8(&info_log).unwrap());
		};
		gl::DeleteShader(vertex_shader);
		gl::DeleteShader(fragment_shader);

		// make vertices
		let vertices: [f32; 9] = [
			-0.5, -0.5, 0.0,
			0.0, -0.5, 0.0,
			-0.25, 0.0, 0.0
		];

		let vertices_two: [f32; 9] = [
			0.0, -0.5, 0.0,
			0.5, -0.5, 0.0,
			0.25, 0.0, 0.0
		];
	
		let (mut vbos, mut vaos) = ([0, 0], [0, 0]);
	
		// we can also generate multiple VAOs or buffers at the same time
		gl::GenVertexArrays(2, vaos.as_mut_ptr());
		gl::GenBuffers(2, vbos.as_mut_ptr());

		// bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
		// first triangle setup
        // --------------------
		gl::BindVertexArray(vaos[0]);

		gl::BindBuffer(gl::ARRAY_BUFFER, vbos[0]);
		gl::BufferData(
			gl::ARRAY_BUFFER,
			(vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
			&vertices[0] as *const f32 as *const c_void,
			gl::STATIC_DRAW,
		);

		gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
		gl::EnableVertexAttribArray(0);

		// second triangle setup
        // ---------------------
		gl::BindVertexArray(vaos[1]);

		gl::BindBuffer(gl::ARRAY_BUFFER, vbos[1]);
        gl::BufferData(
			gl::ARRAY_BUFFER,
            (vertices_two.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices_two[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW
		);

		gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
		gl::EnableVertexAttribArray(0);

		// note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);

		// You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
		gl::BindVertexArray(0);

		// uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

		(shader_program, vbos, vaos)
	};



    // render loop
    // -----------
    while !window.should_close() {
        // events
        // -----
        process_events(&mut window, &events);

        unsafe {
        	gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        	gl::Clear(gl::COLOR_BUFFER_BIT);

			// draw our first triangle
			gl::UseProgram(shader_program);
			// seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
			gl::BindVertexArray(vaos[0]);
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
			gl::BindVertexArray(vaos[1]);
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
			// gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }

	unsafe {
		gl::DeleteVertexArrays(2, vaos.as_mut_ptr());
		gl::DeleteBuffers(2, vbos.as_mut_ptr());
	}
}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                println!("inside close");
                window.set_should_close(true);
            }
            _ => {}
        }
    }
}
