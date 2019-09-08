use wrath::Button;
use wrath::BufferElement;
use wrath::BufferLayout;
use wrath::Indices;
use wrath::Layer;
use wrath::MeshHandle;
use wrath::Renderer;
use wrath::ShaderHandle;
use wrath::ShaderUniform;
use wrath::Vertices;

use std::time::Duration;
use std::time::Instant;

use wrath_math::Float;

pub struct ExampleLayer {
	shader: ShaderHandle,
	meshes: [MeshHandle; 2],
	start_time: Instant,
	rotation: Float,
	rev: bool,
}

impl ExampleLayer {
	pub fn new() -> Self {
		Self {
			shader: ShaderHandle::none(),
			meshes: [MeshHandle::none(); 2],
			start_time: Instant::now(),
			rotation: 0.0,
			rev: false,
		}
	}
}

impl Layer for ExampleLayer {
	fn on_attach(&mut self, renderer: &mut dyn Renderer) {
		let layout = BufferLayout::new(&[
			BufferElement::Vec3,
			BufferElement::Vec4,
		]);
		let indices = Indices::U8(vec![
			0, 1, 2,
			0, 2, 3,
		]);
		self.meshes = [
			renderer.create_mesh(
				&Vertices::new(vec![
					// x     y    z    r    g    b    a
					 0.5,  0.0, 0.0, 0.0, 0.0, 1.0, 1.0,
					 0.0,  0.5, 0.0, 0.0, 1.0, 0.0, 1.0,
					-0.5,  0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
					 0.0, -0.5, 0.0, 1.0, 1.0, 1.0, 0.0,
				]),
				&layout,
				&indices,
			),
			renderer.create_mesh(
				&Vertices::new(vec![
					// x     y     z    r    g    b    a
					 0.25, 0.0,  0.0, 1.0, 1.0, 1.0, 1.0,
					 0.0,  0.25, 0.0, 1.0, 1.0, 1.0, 1.0,
					-0.25, 0.0,  0.0, 1.0, 1.0, 1.0, 1.0,
					 0.0, -0.25, 0.0, 1.0, 1.0, 1.0, 1.0,
				]),
				&layout,
				&indices,
			)
		];

		self.shader = renderer.create_shader(
			std::path::Path::new("sandbox/assets/shaders/cool")
		);

		renderer.bind_shader(self.shader);
	}
	fn on_update(&mut self, dt: Duration) {
		let speed = std::f32::consts::PI * 2.0;
		if Button::E.is_pressed() {
			self.rotation += speed * dt.as_secs_f32();
		}
		if Button::Q.is_pressed() {
			self.rotation -= speed * dt.as_secs_f32();
		}
	}
	fn on_render(&mut self, renderer: &mut dyn Renderer) {
		let elapsed = self.start_time.elapsed().as_secs_f32();
		renderer.set_clear_color((
			elapsed.tan(),
			elapsed.sin(),
			elapsed.cos(),
		).into());

		renderer.set_uniform(
			self.shader,
			"u_rotation",
			ShaderUniform::Float(self.rotation)
		);

		if self.rev {
			for mesh in self.meshes.iter().rev() {
				renderer.render(*mesh, self.shader);
			}
		} else {
			for mesh in self.meshes.iter() {
				renderer.render(*mesh, self.shader);
			}
		}
	}
	fn on_detach(&mut self, renderer: &mut dyn Renderer) {
		for mesh in &self.meshes {
			renderer.delete_mesh(*mesh);
		}
		renderer.delete_shader(self.shader);
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
	fn on_mouse_move(&mut self, position: (u32, u32), delta: (i32, i32)) -> bool {
		println!("Mouse moved to ({}, {}), Δ ({}, {})", position.0, position.1, delta.0, delta.1);
		false
	}
	fn on_mouse_down(&mut self, button: Button) -> bool {
		if button == Button::MouseLeft {
			self.rev = !self.rev;
		}
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