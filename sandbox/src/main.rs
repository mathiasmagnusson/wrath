#![feature(box_syntax)]

use wrath::Button;
use wrath::CallbackHandler;
use wrath::Engine;
use wrath::EngineProps;
use wrath::Layer;
use wrath::LayerHandle;
use wrath::Renderer;
use wrath::ShaderHandle;
use wrath::ShaderUniform;
use wrath::WindowProps;

use std::time::Duration;
use std::time::Instant;

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
		self.ex_layer = engine.push_layer_front(box ExampleLayer::new());
	}
	fn on_update(&mut self, _engine: &mut Engine) {
		// do shit
	}
	fn on_exit(&mut self, engine: &mut Engine) {
		engine.remove_layer(self.ex_layer);
	}
}

struct ExampleLayer {
	vertices: Vec<Float>,
	indices: Vec<u16>,
	va: u32,
	vb: u32,
	ib: u32,
	shader: ShaderHandle,
	start_time: Instant,
}

impl ExampleLayer {
	pub fn new() -> Self {
		Self {
			vertices: vec![
				// x     y    z    r    g    b    a
				 0.5,  0.0, 0.0, 0.0, 0.0, 1.0, 1.0,
				 0.0,  0.5, 0.0, 0.0, 1.0, 0.0, 1.0,
				-0.5,  0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
				 0.0, -0.5, 0.0, 1.0, 1.0, 1.0, 1.0,
			],
			indices: vec![
				0, 1, 2,
				0, 2, 3,
			],
			va: 0,
			vb: 0,
			ib: 0,
			shader: ShaderHandle::none(),
			start_time: Instant::now(),
		}
	}
}

impl Layer for ExampleLayer {
	fn on_attach(&mut self, renderer: &mut dyn Renderer) {
		unsafe {
			renderer.create_shader(
				std::path::Path::new("sandbox/assets/shaders/cool")
			);

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

			// position (x, y, z)
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (size_of::<Float>() * 7) as i32, std::ptr::null_mut());

			// color (r, g, b)
			gl::EnableVertexAttribArray(1);
			gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, (size_of::<Float>() * 7) as i32, (size_of::<Float>() * 3) as _);

			gl::CreateBuffers(1, &mut self.ib);
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ib);

			gl::BufferData(
				gl::ELEMENT_ARRAY_BUFFER,
				size_of_val(self.indices.as_slice()) as isize,
				self.indices.as_ptr() as *const c_void,
				gl::STATIC_DRAW,
			);

			self.shader = renderer.create_shader(std::path::Path::new("sandbox/assets/shaders/cool"));

			renderer.bind_shader(self.shader);
		}
	}
	fn on_update(&mut self, _dt: Duration) {
		// println!("dt: {}", dt.as_secs_f64());
	}
	fn on_render(&mut self, renderer: &mut dyn Renderer) {
		unsafe {
			use wrath::gl;

			let rotation = self.start_time.elapsed().as_secs_f32();

			renderer.set_uniform(
				self.shader,
				"u_rotation",
				ShaderUniform::Float(rotation)
			);

			gl::DrawElements(gl::TRIANGLES, self.indices.len() as _, gl::UNSIGNED_SHORT, std::ptr::null());

			let err = gl::GetError();
			if err != gl::NO_ERROR {
				panic!("Open gl error: {}", match err {
					gl::INVALID_ENUM => "INVALID_ENUM",
					gl::INVALID_VALUE => "INVALID_VALUE",
					gl::INVALID_OPERATION => "INVALID_OPERATION",
					gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
					gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
					gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
					gl::STACK_OVERFLOW => "STACK_OVERFLOW",
					_ => "undefined",
				});
			}
		}
	}
	fn on_detach(&mut self, renderer: &mut dyn Renderer) {
		unsafe {
			use wrath::gl;

			gl::DeleteVertexArrays(1, &self.va);
			gl::DeleteBuffers(1, &self.vb);
			renderer.delete_shader(self.shader);
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
