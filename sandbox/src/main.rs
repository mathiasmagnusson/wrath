#![feature(box_syntax)]

use wrath::Button;
use wrath::CallbackHandler;
use wrath::Engine;
use wrath::EngineProps;
use wrath::Layer;
use wrath::LayerHandle;
use wrath::WindowProps;
use wrath::Renderer;

use std::time::Duration;

use wrath_math::Float;

struct Application {
	ex_layer: LayerHandle,
}

impl Application {
	fn new() -> Self {
		Self {
			ex_layer: LayerHandle::none(),
		}
	}
}

impl CallbackHandler for Application {
	fn on_create(&mut self, engine: &mut Engine) {
		self.ex_layer = engine.layer_stack().push_front(box ExampleLayer::new());
	}
	fn on_update(&mut self, _engine: &mut Engine) {
		// do shit
	}
	fn on_exit(&mut self, engine: &mut Engine) {
		engine.layer_stack().remove_layer(self.ex_layer);
	}
}

struct ExampleLayer {
	vertices: Vec<Float>,
	indices: Vec<usize>,
	v_shad_src: &'static str,
	f_shad_src: &'static str,
	va: u32,
	vb: u32,
	ib: u32,
	sp: u32,
}

impl ExampleLayer {
	pub fn new() -> Self {
		Self {
			vertices: vec![
				-0.5, -0.5, 0.0,
				 0.5, -0.5, 0.0,
				 0.0,  0.5, 0.0,
			],
			indices: vec![
				0, 1, 2,
			],
			v_shad_src: r##"
				#version 330 core
			
				layout(location = 0) in vec3 in_pos;

				void main()
				{
					gl_Position = vec4(in_pos, 1.0);
				}
			"##,
			f_shad_src: r##"
				#version 330 core
			
				layout(location = 0) out vec4 color;

				void main()
				{
					color = vec4(1.0, 1.0, 1.0, 1.0);
				}
			"##,
			va: 0,
			vb: 0,
			ib: 0,
			sp: 0,
		}
	}
}

impl Layer for ExampleLayer {
	fn on_attach(&mut self) {
		unsafe {
			use wrath::gl;
			use std::mem::size_of;
			use std::mem::size_of_val;
			use std::ffi::c_void;

			gl::CreateVertexArrays(1, &mut self.va);
			gl::BindVertexArray(self.va);

			gl::CreateBuffers(1, &mut self.vb);
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vb);

			gl::BufferData(
				gl::ARRAY_BUFFER,
				size_of_val(self.vertices.as_slice()) as isize,
				self.vertices.as_ptr() as *const c_void,
				gl::STATIC_DRAW
			);

			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (size_of::<Float>() * 3) as i32, std::ptr::null_mut());

			gl::CreateBuffers(1, &mut self.ib);
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ib);

			gl::BufferData(
				gl::ELEMENT_ARRAY_BUFFER,
				size_of_val(self.indices.as_slice()) as isize,
				self.indices.as_ptr() as *const c_void,
				gl::STATIC_DRAW,
			);

			let vs = gl::CreateShader(gl::VERTEX_SHADER);
			let ptr = self.v_shad_src.as_ptr() as *const i8;
			let len = self.v_shad_src.len() as i32;
			gl::ShaderSource(vs, 1, &ptr, &len);

			gl::CompileShader(vs);

			let mut success = 0;
			gl::GetShaderiv(vs, gl::COMPILE_STATUS, &mut success);
			if success == 0 {
				println!("\x1b[31mVertex Shader Compilation Failiure:\x1b[0m");

				let mut len = 0;
				gl::GetShaderiv(vs, gl::INFO_LOG_LENGTH, &mut len);

				let mut log = vec![b'a'; len as usize - 1];
				gl::GetShaderInfoLog(vs, len, &mut len, log.as_mut_ptr() as *mut i8);

				let log = String::from_utf8_lossy(&log);

				println!("{}", log);
				panic!();
			}

			let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
			let ptr = self.f_shad_src.as_ptr() as *const i8;
			let len = self.f_shad_src.len() as i32;
			gl::ShaderSource(fs, 1, &ptr, &len);

			gl::CompileShader(fs);

			let mut success = 0;
			gl::GetShaderiv(fs, gl::COMPILE_STATUS, &mut success);
			if success == 0 {
				println!("\x1b[31mFragment Shader Compilation Failiure:\x1b[0m");

				let mut len = 0;
				gl::GetShaderiv(fs, gl::INFO_LOG_LENGTH, &mut len);

				let mut log = vec![b'a'; len as usize - 1];
				gl::GetShaderInfoLog(fs, len, &mut len, log.as_mut_ptr() as *mut i8);

				let log = String::from_utf8_lossy(&log);

				println!("{}", log);
				panic!();
			}

			self.sp = gl::CreateProgram();

			gl::AttachShader(self.sp, vs);
			gl::AttachShader(self.sp, fs);
			gl::LinkProgram(self.sp);

			let mut success = 0;
			gl::GetProgramiv(self.sp, gl::LINK_STATUS, &mut success);
			if success == 0 {
				println!("\x1b[31mShader Program Linking Failiure:\x1b[0m");

				let mut len = 0;
				gl::GetProgramiv(self.sp, gl::INFO_LOG_LENGTH, &mut len);

				let mut log = Vec::<u8>::with_capacity(len as usize);
				gl::GetProgramInfoLog(self.sp, len, &mut len, log.as_mut_ptr() as *mut i8);

				let log = String::from_utf8_lossy(&log);

				println!("{}", log);
				panic!();
			}

			gl::DetachShader(self.sp, vs);
			gl::DetachShader(self.sp, fs);

			gl::UseProgram(self.sp);
		}

		println!("va: {} vb: {} ib: {} sp: {}", self.va, self.vb, self.ib, self.sp);
	}
	fn on_update(&mut self, _dt: Duration) {
		// println!("dt: {}", dt.as_secs_f64());
	}
	fn on_render(&mut self, _renderer: &mut Renderer) {
		unsafe {
			use wrath::gl;

			gl::DrawElements(gl::TRIANGLES, 6, self.indices.len() as _, std::ptr::null());
		}
	}
	fn on_detach(&mut self) {
		unsafe {
			use wrath::gl;

			gl::DeleteVertexArrays(1, &self.va);
			gl::DeleteBuffers(1, &self.vb);
			gl::DeleteProgram(self.sp);
		}
	}
	fn on_window_resize(&mut self, size: (u32, u32)) {
		println!("Window resized: ({}, {})", size.0, size.1);
	}
	fn on_text_written(&mut self, which: char) -> bool {
		println!("{}", which);
		false
	}
	fn on_key_press(&mut self, button: Button, repeat: bool) -> bool {
		println!("Key pressed: {:?} {}", button, if repeat { "again" } else { "" });
		false
	}
	fn on_key_release(&mut self, button: Button) -> bool {
		println!("Key released: {:?}", button);
		false
	}
	// fn on_mouse_move(&mut self, position: (u32, u32), delta: (i32, i32)) -> bool {
	// 	println!("Mouse moved to ({}, {}), Δ ({}, {})", position.0, position.1, delta.0, delta.1);
	// 	false
	// }
	fn on_mouse_down(&mut self, button: Button) -> bool {
		println!("Click {:?}!", button);
		false
	}
	fn on_mouse_up(&mut self, button: Button) -> bool {
		println!("Click {:?}¡", button);
		false
	}
	fn on_mouse_scroll(&mut self, delta: (Float, Float)) -> bool {
		println!("Scroll: ({}, {})", delta.0, delta.1);
		false
	}
}

fn main() {
	wrath::init(Application::new(), EngineProps {
		window_props: WindowProps {
			title: "Curls of Lordraft".into(),
			size: (1080, 720),
		}
	});
}
