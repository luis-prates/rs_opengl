extern crate glfw;

use cgmath::{Matrix4, vec3, Deg, perspective, Point3, InnerSpace};
use glfw::{Key, Action};

use crate::shader;
use crate::camera;
use crate::common;
use crate::model;

use self::glfw::Context;

extern crate gl;

use std::ffi::{CStr, CString };

use common::{process_events, process_input};
use shader::Shader;
use camera::Camera;
use model::Model;

extern crate image;

enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_3_2() {
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
    // let light_pos = vec3(1.2, 1.0, 2.0);

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

	let (our_shader, our_model, our_model2) = unsafe {

		gl::Enable(gl::DEPTH_TEST);

		let our_shader = Shader::new(
			"src/_3_model_loading/shaders/1.model_loading_42.vs", 
			"src/_3_model_loading/shaders/1.model_loading_42.fs"
		);

		// load models
		let our_model = Model::new("resources/textures/42.obj");
		let our_model2: Model = Model::new("resources/objects/planet/planet_offset_down_more.obj");
		// let our_model: Model = Model::new("resources/objects/nanosuit/nanosuit.obj");

		// draw in wireframe
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

		(our_shader, our_model, our_model2)
	};



    // // render loop
	// unsafe { 
	// 	our_shader.use_program();
	// 	our_shader.set_int(CStr::from_bytes_with_nul(b"texture1\0").unwrap(), 0);
	// 	our_shader.set_int(CStr::from_bytes_with_nul(b"texture2\0").unwrap(), 1);
	// }

	let mut position_x = 0.0;
	let mut position_y = 0.0;
	let mut position_z = 0.0;
	let mut use_color = 0;
	let mut mix_value = 0.0;
	let delay_time = 1.0;
	let mut last_time: f32 = 0.0;

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

		// process_input(&mut window, delta_time, &mut camera);
		let velocity = 2.5 * delta_time;
		if window.get_key(Key::Escape) == Action::Press {
			window.set_should_close(true)
		}
		if window.get_key(Key::W) == Action::Press {
			position_y += 1.0 * velocity; 
		}
		if window.get_key(Key::S) == Action::Press {
			position_y -= 1.0 * velocity; 
		}
		if window.get_key(Key::A) == Action::Press {
			position_x -= 1.0 * velocity; 
		}
		if window.get_key(Key::D) == Action::Press {
			position_x += 1.0 * velocity;
		}
		if window.get_key(Key::Q) == Action::Press {
			position_z -= 1.0 * velocity; 
		}
		if window.get_key(Key::E) == Action::Press {
			position_z += 1.0 * velocity;
		}
		if window.get_key(Key::Down) == Action::Press {
			mix_value -= 0.01;
			if mix_value <= 0.0 {
				mix_value = 0.0;
			}
			println!("mix value: {}", mix_value);

		}
		if window.get_key(Key::Up) == Action::Press {
			mix_value += 0.01;
			if mix_value >= 1.0 {
				mix_value = 1.0;
			}
			println!("mix value: {}", mix_value);
		}
		
		unsafe {
			gl::ClearColor(0.1, 0.1, 0.1, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
			
			let use_texturing = CString::new("useTexturing").unwrap();
			let use_mix = CString::new("mixValue").unwrap();
			if window.get_key(Key::Enter) == Action::Press && glfw.get_time() as f32 - last_time > delay_time {
				use_color ^= 1;
				println!("use color value: {}", use_color);
				last_time = glfw.get_time() as f32;
			}
			if use_color == 1 {
				mix_value += 0.005;
				if mix_value >= 1.0 {
					mix_value = 1.0;
				}
			}
			else {
				mix_value -= 0.005;
				if mix_value <= 0.0 {
					mix_value = 0.0;
				}
			}
			gl::Uniform1i(gl::GetUniformLocation(our_shader.id, use_texturing.as_ptr()), use_color);
			gl::Uniform1f(gl::GetUniformLocation(our_shader.id, use_mix.as_ptr()), mix_value);

			// be sure to activate shader when setting uniforms/drawing objects
			our_shader.use_program();

			let projection: Matrix4<f32> = perspective(Deg(camera.zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
			let view = camera.get_view_matrix();

			// get matrix's uniform location and set matrix
			our_shader.set_mat4(CStr::from_bytes_with_nul(b"view\0").unwrap(), &view);
			our_shader.set_mat4(CStr::from_bytes_with_nul(b"projection\0").unwrap(), &projection);

			// render the loaded model
			let (center_x, center_y, center_z) = our_model.get_center_all_axes();
			let angle = glfw.get_time() as f32 * 50.0;
			let mut model = Matrix4::from_scale(0.2);
			model = model * Matrix4::<f32>::from_translation(vec3(position_x, position_y, position_z));
			model = model * Matrix4::from_axis_angle(vec3(0.0, 1.0, 0.0).normalize(), Deg(angle));
			model = model * Matrix4::<f32>::from_translation(vec3(-center_x, -center_y, -center_z));

			our_shader.set_mat4(CStr::from_bytes_with_nul(b"model\0").unwrap(), &model);
			our_model.draw(&our_shader);

			gl::Uniform1i(gl::GetUniformLocation(our_shader.id, use_texturing.as_ptr()), 1);
			gl::Uniform1f(gl::GetUniformLocation(our_shader.id, use_mix.as_ptr()), 0.0);

			let (center_x, center_y, center_z) = our_model2.get_center_all_axes();
			let mut model = Matrix4::from_scale(0.2);
			model = model * Matrix4::<f32>::from_translation(vec3(5.0, 1.75, 0.0));
			model = model * Matrix4::from_axis_angle(vec3(1.0, 0.0, 0.0), Deg(angle));
			model = model * Matrix4::from_translation(vec3(-center_x, -center_y, -center_z));
			our_shader.set_mat4(CStr::from_bytes_with_nul(b"model\0").unwrap(), &model);
			our_model2.draw(&our_shader);

        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}
