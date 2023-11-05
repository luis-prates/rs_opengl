extern crate glfw;

use cgmath::{Matrix4, Vector3, vec3, Deg, perspective, Point3};
use cgmath::prelude::*;
use gl::types::{GLfloat, GLsizeiptr};

use crate::shader;

use self::glfw::{Action, Context, Key};

extern crate gl;
use self::gl::types::*;

use std::{mem, ptr};
use std::os::raw::c_void;
use std::sync::mpsc::Receiver;
use std::path::Path;
use std::ffi::{CStr, CString};

use shader::Shader;

extern crate image;
use image::GenericImage;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

// camera
const CAMERA_UP: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub fn main_1_7_3() {

	let mut mix_value = 0.5;

	let mut camera_pos = Point3::new(0.0, 0.0, 3.0);
	let mut camera_front: Vector3<f32> = Vector3 {
		x: 0.0,
		y: 0.0,
		z: -1.0,
	};

	let mut first_mouse = true;
    let mut yaw: f32 = -90.0; // yaw is initialized to -90.0 degrees since a yaw of 0.0 results in a direction vector pointing to the right so we initially rotate a bit to the left.
    let mut pitch: f32 = 0.0;
    let mut last_x: f32 = SCR_WIDTH as f32 / 2.0;
    let mut last_y: f32 = SCR_HEIGHT as f32 / 2.0;
    let mut fov: f32 = 45.0;

	// timing
    let mut delta_time: f32; // time between current frame and last frame
    let mut last_frame: f32 = 0.0;

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
    //window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
	window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);


	window.set_cursor_mode(glfw::CursorMode::Disabled);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

	let (our_shader, vao, texture1, texture2, cube_positions) = unsafe {

		gl::Enable(gl::DEPTH_TEST);

		let our_shader = Shader::new(
			"src/_1_getting_started/shaders/6.1.coordinates.vs", 
			"src/_1_getting_started/shaders/6.1.coordinates.fs"
		);

		// make vertices
		let vertices: [f32; 180] = [
			-0.5, -0.5, -0.5,  0.0, 0.0,
			0.5, -0.5, -0.5,  1.0, 0.0,
			0.5,  0.5, -0.5,  1.0, 1.0,
			0.5,  0.5, -0.5,  1.0, 1.0,
			-0.5,  0.5, -0.5,  0.0, 1.0,
			-0.5, -0.5, -0.5,  0.0, 0.0,

			-0.5, -0.5,  0.5,  0.0, 0.0,
			0.5, -0.5,  0.5,  1.0, 0.0,
			0.5,  0.5,  0.5,  1.0, 1.0,
			0.5,  0.5,  0.5,  1.0, 1.0,
			-0.5,  0.5,  0.5,  0.0, 1.0,
			-0.5, -0.5,  0.5,  0.0, 0.0,

			-0.5,  0.5,  0.5,  1.0, 0.0,
			-0.5,  0.5, -0.5,  1.0, 1.0,
			-0.5, -0.5, -0.5,  0.0, 1.0,
			-0.5, -0.5, -0.5,  0.0, 1.0,
			-0.5, -0.5,  0.5,  0.0, 0.0,
			-0.5,  0.5,  0.5,  1.0, 0.0,

			0.5,  0.5,  0.5,  1.0, 0.0,
			0.5,  0.5, -0.5,  1.0, 1.0,
			0.5, -0.5, -0.5,  0.0, 1.0,
			0.5, -0.5, -0.5,  0.0, 1.0,
			0.5, -0.5,  0.5,  0.0, 0.0,
			0.5,  0.5,  0.5,  1.0, 0.0,

			-0.5, -0.5, -0.5,  0.0, 1.0,
			0.5, -0.5, -0.5,  1.0, 1.0,
			0.5, -0.5,  0.5,  1.0, 0.0,
			0.5, -0.5,  0.5,  1.0, 0.0,
			-0.5, -0.5,  0.5,  0.0, 0.0,
			-0.5, -0.5, -0.5,  0.0, 1.0,

			-0.5,  0.5, -0.5,  0.0, 1.0,
			0.5,  0.5, -0.5,  1.0, 1.0,
			0.5,  0.5,  0.5,  1.0, 0.0,
			0.5,  0.5,  0.5,  1.0, 0.0,
			-0.5,  0.5,  0.5,  0.0, 0.0,
			-0.5,  0.5, -0.5,  0.0, 1.0
		];

		let cube_positions: [Vector3<f32>; 10] = [
			vec3(0.0, 0.0, 0.0),
			vec3(2.0, 5.0, -15.0),
			vec3(-1.5, -2.2, -2.5),
			vec3(-3.8, -2.0, -12.3),
			vec3(2.4, -0.4, -3.5),
			vec3(-1.7, 3.0, -7.5),
			vec3(1.3, -2.0, -2.5),
			vec3(1.5, 2.0, -2.5),
			vec3(1.5, 0.2, -1.5),
			vec3(-1.3, 1.0, -1.5)
		];
	
		let (mut vbo, mut vao) = (0, 0);
	
		// we can also generate multiple VAOs or buffers at the same time
		gl::GenVertexArrays(1, &mut vao);
		gl::GenBuffers(1, &mut vbo);

		// bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
		// first triangle setup
        // --------------------
		gl::BindVertexArray(vao);

		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER,
			(vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
			&vertices[0] as *const f32 as *const c_void,
			gl::STATIC_DRAW,
		);

		let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
		// position attribute
		gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
		gl::EnableVertexAttribArray(0);

		// texture coord attribute
		gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
		gl::EnableVertexAttribArray(1);

		// uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

		let mut texture1 = 0;
		gl::GenTextures(1, &mut texture1);
		gl::BindTexture(gl::TEXTURE_2D, texture1); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
		// set the texture wrapping parameters
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
		// set the texture filtering parameters
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
		// load image, create texture and generate mipmaps
		let img = image::open(&Path::new("resources/textures/container.jpg")).expect("Failed to load texture");
		let data = img.raw_pixels();
		gl::TexImage2D(
			gl::TEXTURE_2D,
			0,
			gl::RGB as i32,
			img.width() as i32,
			img.height() as i32,
			0,
			gl::RGB,
			gl::UNSIGNED_BYTE,
			&data[0] as *const u8 as *const c_void
		);
		gl::GenerateMipmap(gl::TEXTURE_2D);

		let mut texture2 = 0;
		gl::GenTextures(1, &mut texture2);
		gl::BindTexture(gl::TEXTURE_2D, texture2); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
		// set the texture wrapping parameters
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
		// set the texture filtering parameters
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
		// load image, create texture and generate mipmaps
		let img = image::open(&Path::new("resources/textures/awesomeface.png")).expect("Failed to load texture");
		let img = img.flipv();
		let data = img.raw_pixels();
		gl::TexImage2D(
			gl::TEXTURE_2D,
			0,
			gl::RGB as i32,
			img.width() as i32,
			img.height() as i32,
			0,
			gl::RGBA,
			gl::UNSIGNED_BYTE,
			&data[0] as *const u8 as *const c_void
		);
		gl::GenerateMipmap(gl::TEXTURE_2D);


		(our_shader, vao, texture1, texture2, cube_positions)
	};



    // render loop
	unsafe { 
		our_shader.use_program();
		our_shader.set_int(CStr::from_bytes_with_nul(b"texture1\0").unwrap(), 0);
		our_shader.set_int(CStr::from_bytes_with_nul(b"texture2\0").unwrap(), 1);
	}

    // -----------
    while !window.should_close() {

		let current_frame = glfw.get_time() as f32;
		delta_time = current_frame - last_frame;
		last_frame = current_frame;

        // events
        // -----
        process_events(
			&events,
			&mut mix_value,
			&mut first_mouse,
			&mut last_x,
			&mut last_y,
			&mut yaw,
			&mut pitch,
			&mut camera_front,
			&mut fov
		);

		process_input(&mut window, delta_time, &mut camera_pos, &mut camera_front);

		unsafe {
			gl::ClearColor(0.2, 0.3, 0.3, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

			// render the triangle
			gl::ActiveTexture(gl::TEXTURE0);
			gl::BindTexture(gl::TEXTURE_2D, texture1);
			gl::ActiveTexture(gl::TEXTURE1);
			gl::BindTexture(gl::TEXTURE_2D, texture2);
			let offset_cstring = CString::new("mixValue").unwrap();
			let offset_cstr = CStr::from_bytes_with_nul(offset_cstring.as_bytes_with_nul())
				.expect("CStr conversion failed");

			our_shader.set_float(offset_cstr, mix_value);

			// create transformations
			//let model: Matrix4<f32> = Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0).normalize(), Rad(glfw.get_time() as f32));
			let projection: Matrix4<f32> = perspective(Deg(fov), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
			
			let view: Matrix4<f32>;
			let eye = camera_pos;
			let center = camera_pos + camera_front;
			view = Matrix4::look_at(eye, center, CAMERA_UP);

			// get matrix's uniform location and set matrix
			our_shader.use_program();
			//our_shader.set_mat4(CStr::from_bytes_with_nul(b"model\0").unwrap(), &model);
			our_shader.set_mat4(CStr::from_bytes_with_nul(b"view\0").unwrap(), &view);
			our_shader.set_mat4(CStr::from_bytes_with_nul(b"projection\0").unwrap(), &projection);


			// seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
			gl::BindVertexArray(vao);

			for (i, position) in cube_positions.iter().enumerate() {
				// calculate the model matrix for each object and pass it to the shader before drawing
				let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
				let mut angle = 20.0 * i as f32;
				if i % 3 == 0 {
					angle = glfw.get_time() as f32 * 25.0;
				}
				model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
				our_shader.set_mat4(CStr::from_bytes_with_nul(b"model\0").unwrap(), &model);
				gl::DrawArrays(gl::TRIANGLES, 0, 36)
			}

        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}

// NOTE: not the same version as in common.rs!
fn process_events(
	events: &Receiver<(f64, glfw::WindowEvent)>,
	mix_value: &mut f32,
	first_mouse: &mut bool,
	last_x: &mut f32,
	last_y: &mut f32,
	yaw: &mut f32,
	pitch: &mut f32,
	camera_front: &mut Vector3<f32>,
	fov: &mut f32
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
			glfw::WindowEvent::Key(Key::Down, _, Action::Repeat, _) => {
                *mix_value -= 0.01;
				if *mix_value <= 0.0 {
					*mix_value = 0.0;
				}
            }
			glfw::WindowEvent::Key(Key::Up, _, Action::Repeat, _) => {
                *mix_value += 0.01;
				if *mix_value >= 1.0 {
					*mix_value = 1.0;
				}
            }
			glfw::WindowEvent::CursorPos(xpos, ypos) => {
                let (xpos, ypos) = (xpos as f32, ypos as f32);
                if *first_mouse {
                    *last_x = xpos;
                    *last_y = ypos;
                    *first_mouse = false;
                }

                let mut xoffset = xpos - *last_x;
                let mut yoffset = *last_y - ypos; // reversed since y-coordinates go from bottom to top
                *last_x = xpos;
                *last_y = ypos;

                let sensitivity: f32 = 0.01; // change this value to your liking
                xoffset *= sensitivity;
                yoffset *= sensitivity;

                *yaw += xoffset;
                *pitch += yoffset;

                // make sure that when pitch is out of bounds, screen doesn't get flipped
                if *pitch > 89.0 {
                    *pitch = 89.0;
                }
                if *pitch < -89.0 {
                    *pitch = -89.0;
                }

                let front = Vector3 {
                    x: yaw.to_radians().cos() * pitch.to_radians().cos(),
                    y: pitch.to_radians().sin(),
                    z: yaw.to_radians().sin() * pitch.to_radians().cos(),
                };
                *camera_front = front.normalize();
            }
			glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                if *fov >= 1.0 && *fov <= 45.0 {
                    *fov -= yoffset as f32;
                }
                if *fov <= 1.0 {
                    *fov = 1.0;
                }
                if *fov >= 45.0 {
                    *fov = 45.0;
                }
            }
            _ => {}
        }
    }
}

fn process_input(window: &mut glfw::Window, delta_time: f32, camera_pos: &mut Point3<f32>, camera_front: &mut Vector3<f32>) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }

    let camera_speed = 2.5 * delta_time;
    if window.get_key(Key::W) == Action::Press {
        *camera_pos += camera_speed * *camera_front;
    }
    if window.get_key(Key::S) == Action::Press {
        *camera_pos += -(camera_speed * *camera_front);
    }
    if window.get_key(Key::A) == Action::Press {
        *camera_pos += -(camera_front.cross(CAMERA_UP).normalize() * camera_speed);
    }
    if window.get_key(Key::D) == Action::Press {
        *camera_pos += camera_front.cross(CAMERA_UP).normalize() * camera_speed;
    }

}

