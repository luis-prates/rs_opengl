extern crate glfw;

use cgmath::{Matrix4, Vector3, vec3, Deg, perspective, Point3};
use cgmath::prelude::*;
use gl::types::{GLfloat, GLsizeiptr};

use crate::shader;
use crate::camera;
use crate::common;

use self::glfw::Context;

extern crate gl;
use self::gl::types::*;

use std::{mem, ptr};
use std::os::raw::c_void;
use std::ffi::CStr;

use common::{process_events, process_input};
use shader::Shader;
use camera::Camera;

extern crate image;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_2_3_1() {
	let mut camera = Camera {
		position: Point3::new(0.0, 0.0, 3.0),
		..Camera::default()
	};

	let mut first_mouse = true;
    let mut last_x: f32 = SCR_WIDTH as f32 / 2.0;
    let mut last_y: f32 = SCR_HEIGHT as f32 / 2.0;

	// timing
    let mut delta_time: f32; // time between current frame and last frame
    let mut last_frame: f32 = 0.0;

	// lighting
    let light_pos = vec3(1.2, 1.0, 2.0);

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

	let (lighting_shader, lamp_shader, vbo, cube_vao, cube_positions, light_vao) = unsafe {

		gl::Enable(gl::DEPTH_TEST);

		let lighting_shader = Shader::new(
			"src/_2_lighting/shaders/3.1.materials.vs", 
			"src/_2_lighting/shaders/3.1.materials.fs"
		);

		let lamp_shader = Shader::new(
			"src/_2_lighting/shaders/1.lamp.vs",
			"src/_2_lighting/shaders/1.lamp.fs"
		);

		// make vertices
		let vertices: [f32; 216] = [
			-0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
			0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
			0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
			0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
			-0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
			-0.5, -0.5, -0.5,  0.0,  0.0, -1.0,

			-0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
			0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
			0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
			0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
			-0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
			-0.5, -0.5,  0.5,  0.0,  0.0,  1.0,

			-0.5,  0.5,  0.5, -1.0,  0.0,  0.0,
			-0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
			-0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
			-0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
			-0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
			-0.5,  0.5,  0.5, -1.0,  0.0,  0.0,

			0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
			0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
			0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
			0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
			0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
			0.5,  0.5,  0.5,  1.0,  0.0,  0.0,

			-0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
			0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
			0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
			0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
			-0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
			-0.5, -0.5, -0.5,  0.0, -1.0,  0.0,

			-0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
			0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
			0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
			0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
			-0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
			-0.5,  0.5, -0.5,  0.0,  1.0,  0.0
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
	
		let (mut vbo, mut cube_vao, mut light_vao) = (0, 0, 0);
	
		// we can also generate multiple VAOs or buffers at the same time
		gl::GenVertexArrays(1, &mut cube_vao);
		gl::GenVertexArrays(1, &mut light_vao);
		gl::GenBuffers(1, &mut vbo);

		// bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
		// first triangle setup
        // --------------------
		gl::BindVertexArray(cube_vao);

		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER,
			(vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
			&vertices[0] as *const f32 as *const c_void,
			gl::STATIC_DRAW,
		);

		let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
		// position attribute
		gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
		gl::EnableVertexAttribArray(0);

		// normal vector attribute
		gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
		gl::EnableVertexAttribArray(1);

		gl::BindVertexArray(light_vao);
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

		// lamp coord attribute
		gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
		gl::EnableVertexAttribArray(0);


		// uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

		(lighting_shader, lamp_shader, vbo, cube_vao, cube_positions, light_vao)
	};



    // // render loop
	// unsafe { 
	// 	lighting_shader.use_program();
	// 	lighting_shader.set_int(CStr::from_bytes_with_nul(b"texture1\0").unwrap(), 0);
	// 	lighting_shader.set_int(CStr::from_bytes_with_nul(b"texture2\0").unwrap(), 1);
	// }

    // -----------
    while !window.should_close() {

		let current_frame = glfw.get_time() as f32;
		delta_time = current_frame - last_frame;
		last_frame = current_frame;

        // events
        // -----
        process_events(
			&events,
			&mut first_mouse,
			&mut last_x,
			&mut last_y,
			&mut camera
		);

		process_input(&mut window, delta_time, &mut camera);

		unsafe {
			gl::ClearColor(0.2, 0.3, 0.3, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

			// be sure to activate shader when setting uniforms/drawing objects
			lighting_shader.use_program();

			// material properties
			lighting_shader.set_vec3(CStr::from_bytes_with_nul(b"material.ambient\0").unwrap(), 1.0, 0.5, 0.31);
			lighting_shader.set_vec3(CStr::from_bytes_with_nul(b"material.diffuse\0").unwrap(), 1.0, 0.5, 0.31);
			lighting_shader.set_vec3(CStr::from_bytes_with_nul(b"material.specular\0").unwrap(), 0.5, 0.5, 0.5);
			// emerald
			// lighting_shader.set_vec3(CStr::from_bytes_with_nul(b"material.ambient\0").unwrap(), 0.0215, 0.1745, 0.0215);
			// lighting_shader.set_vec3(CStr::from_bytes_with_nul(b"material.diffuse\0").unwrap(), 0.07568, 0.61424, 0.07568);
			// lighting_shader.set_vec3(CStr::from_bytes_with_nul(b"material.specular\0").unwrap(), 0.633, 0.316228, 0.316228);
			lighting_shader.set_float(CStr::from_bytes_with_nul(b"material.shininess\0").unwrap(), 32.0);

			// light properties
			let time = glfw.get_time() as f32;
			let light_color: Vector3<f32> = Vector3 {
				x: f32::sin(time * 2.0),
				y: f32::sin(time * 0.7),
				z: f32::sin(time * 1.3),

			};

			let diffuse_color = light_color * 0.5;
			let ambient_color = diffuse_color * 0.2;
			lighting_shader.set_vector3(CStr::from_bytes_with_nul(b"light.ambient\0").unwrap(), &ambient_color);
			lighting_shader.set_vector3(CStr::from_bytes_with_nul(b"light.diffuse\0").unwrap(), &diffuse_color);
			lighting_shader.set_vec3(CStr::from_bytes_with_nul(b"light.specular\0").unwrap(), 1.0, 1.0, 1.0);
			lighting_shader.set_vector3(CStr::from_bytes_with_nul(b"light.position\0").unwrap(), &light_pos);
			lighting_shader.set_vector3(CStr::from_bytes_with_nul(b"viewPos\0").unwrap(), &vec3(camera.position.x, camera.position.y, camera.position.z));

			// create transformations
			//let model: Matrix4<f32> = Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0).normalize(), Rad(glfw.get_time() as f32));
			let projection: Matrix4<f32> = perspective(Deg(camera.zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
			
			let view = camera.get_view_matrix();

			// get matrix's uniform location and set matrix
			lighting_shader.set_mat4(CStr::from_bytes_with_nul(b"view\0").unwrap(), &view);
			lighting_shader.set_mat4(CStr::from_bytes_with_nul(b"projection\0").unwrap(), &projection);

			// seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
			gl::BindVertexArray(cube_vao);

			for (i, position) in cube_positions.iter().enumerate() {
				// calculate the model matrix for each object and pass it to the shader before drawing
				let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
				let mut angle = 20.0 * i as f32;
				if i % 3 == 0 {
					angle = glfw.get_time() as f32 * 25.0;
				}
				model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
				lighting_shader.set_mat4(CStr::from_bytes_with_nul(b"model\0").unwrap(), &model);
				gl::DrawArrays(gl::TRIANGLES, 0, 36)
			}

			lamp_shader.use_program();
			lamp_shader.set_vector3(CStr::from_bytes_with_nul(b"light.diffuse\0").unwrap(), &diffuse_color);
			lamp_shader.set_mat4(CStr::from_bytes_with_nul(b"projection\0").unwrap(), &projection);
			lamp_shader.set_mat4(CStr::from_bytes_with_nul(b"view\0").unwrap(), &view);
			// light_pos.x = 1.0 + f32::sin(glfw.get_time() as f32) * 2.0;
			// light_pos.y = f32::sin(glfw.get_time() as f32 / 2.0) * 1.0;
			let mut model = Matrix4::from_translation(light_pos);
			model = model * Matrix4::from_scale(0.2);
			lamp_shader.set_mat4(CStr::from_bytes_with_nul(b"model\0").unwrap(), &model);

			gl::BindVertexArray(light_vao);
			gl::DrawArrays(gl::TRIANGLES, 0, 36);

        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }

	// optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteVertexArrays(1, &cube_vao);
        gl::DeleteVertexArrays(1, &light_vao);
        gl::DeleteBuffers(1, &vbo);
    }
}
