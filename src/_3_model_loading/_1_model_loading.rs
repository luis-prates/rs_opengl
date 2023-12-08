extern crate glfw;

use cgmath::{Matrix4, vec3, Deg, perspective, Point3};

use crate::shader;
use crate::camera;
use crate::common;
use crate::model;

use self::glfw::Context;

extern crate gl;

use std::ffi::CStr;

use common::{process_events, process_input};
use shader::Shader;
use camera::Camera;
use model::Model;

extern crate image;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_3_1() {
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
			"src/_3_model_loading/shaders/1.model_loading.vs", 
			"src/_3_model_loading/shaders/1.model_loading.fs"
		);

		// load models
		let our_model = Model::new("resources/textures/42.obj");
		let our_model2: Model = Model::new("resources/objects/planet/planet.obj");

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
			gl::ClearColor(0.1, 0.1, 0.1, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

			// be sure to activate shader when setting uniforms/drawing objects
			our_shader.use_program();

			let projection: Matrix4<f32> = perspective(Deg(camera.zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
			let view = camera.get_view_matrix();

			// get matrix's uniform location and set matrix
			our_shader.set_mat4(CStr::from_bytes_with_nul(b"view\0").unwrap(), &view);
			our_shader.set_mat4(CStr::from_bytes_with_nul(b"projection\0").unwrap(), &projection);

			// render the loaded model
			let mut model = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0)); // translate it down so it's at the center of the scene
			model = model * Matrix4::from_scale(0.2);
			our_shader.set_mat4(CStr::from_bytes_with_nul(b"model\0").unwrap(), &model);
			our_model.draw(&our_shader);
			let mut model = Matrix4::<f32>::from_translation(vec3(5.0, 1.75, 0.0)); // translate it down so it's at the center of the scene
			model = model * Matrix4::from_scale(0.2);
			our_shader.set_mat4(CStr::from_bytes_with_nul(b"model\0").unwrap(), &model);
			our_model2.draw(&our_shader);

        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}
