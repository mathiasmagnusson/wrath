#![feature(box_syntax)]

// mod example_overlay;
// use example_overlay::ExampleOverlay;

use std::time::Duration;
use std::path::Path;

fn main() {
	wrath::init(Application::new(), wrath::EngineProps {
		window_props: wrath::WindowProps {
			title: "Curls of Lordraft".into(),
			size: (800, 500),
		}
	});
}

struct Application {
	ex_overlay: wrath::OverlayHandle,
}

impl Application {
	fn new() -> Self {
		Self {
			ex_overlay: wrath::OverlayHandle::none(),
		}
	}
}

impl wrath::CallbackHandler for Application {
	fn on_create(&mut self, engine: &mut wrath::Engine) {
		self.ex_overlay = engine.push_overlay_front(box SnakeOverlay::new());
	}
	fn on_update(&mut self, _engine: &mut wrath::Engine) {
		// do shit
	}
	fn on_exit(&mut self, engine: &mut wrath::Engine) {
		engine.remove_overlay(self.ex_overlay);
	}
}

const COLS: u32 = 16;
const ROWS: u32 = 10;
const SNAKE_COLOR: whmath::Vec4 = whmath::Vec4::new(0.0, 1.0, 0.0, 1.0);
const FRUIT_COLOR: whmath::Vec4 = whmath::Vec4::new(1.0, 0.0, 0.0, 1.0);

struct SnakeOverlay {
	cube_mesh: wrath::MeshHandle,
	flat_cube_shader: wrath::ShaderHandle,
	elapsed: Duration,
	frame_time: Duration,
	snake: Vec<(u32, u32)>,
	dir: u8,
	turned: bool,
	fruit: (u32, u32),
}

impl SnakeOverlay {
	fn new() -> Self {
		Self {
			cube_mesh: wrath::MeshHandle::none(),
			flat_cube_shader: wrath::ShaderHandle::none(),
			elapsed: Duration::new(0, 0),
			frame_time: Duration::from_millis(200),
			snake: vec![(1, 1)],
			dir: 0,
			turned: false,
			fruit: (
				rand::random::<u32>() % COLS,
				rand::random::<u32>() % ROWS,
			)
		}
	}
	fn replace_fruit(&mut self) {
		self.fruit = (
			rand::random::<u32>() % COLS,
			rand::random::<u32>() % ROWS,
		);
	}
	fn step(&mut self) {
		self.turned = false;
		let delta = match self.dir {
			0 => ( 1,  0),
			1 => ( 0,  1),
			2 => (-1,  0),
			3 => ( 0, -1),
			_ => panic!(),
		};
		let head = self.snake.last().unwrap();
		let new_pos = (
			(head.0 as i32 + delta.0),
			(head.1 as i32 + delta.1),
		);
		if new_pos.0 < 0 ||
			new_pos.1 < 0 ||
			new_pos.0 == COLS as _ ||
			new_pos.1 == ROWS as _
		{
			return self.game_over();
		}
		let new_pos = (
			new_pos.0 as u32,
			new_pos.1 as u32,
		);
		if self.snake.contains(&new_pos) {
			return self.game_over();
		}
		self.snake.push(new_pos);
		let ate = new_pos == self.fruit;
		if ate {
			self.replace_fruit();
		}
		else {
			self.snake.remove(0);
		}
	}
	fn game_over(&mut self) {
		self.snake = vec![(1, 1)];
		self.dir = 0;
		self.replace_fruit();
	}
}

impl wrath::Overlay for SnakeOverlay {
	fn on_update(&mut self, dt: Duration) {
		if !self.turned {
			if wrath::Button::ArrowRight.is_pressed() {
				self.dir = 0;
				self.turned = true;
			}
			if wrath::Button::ArrowUp.is_pressed() {
				self.dir = 1;
				self.turned = true;
			}
			if wrath::Button::ArrowLeft.is_pressed() {
				self.dir = 2;
				self.turned = true;
			}
			if wrath::Button::ArrowDown.is_pressed() {
				self.dir = 3;
				self.turned = true;
			}
		}

		if wrath::Button::Q.is_pressed() {
			std::process::exit(0);
		}

		self.elapsed += dt;
		if self.elapsed >= self.frame_time {
			self.step();
			self.elapsed -= self.frame_time;
		}
	}
	fn on_attach(&mut self, renderer: &mut dyn wrath::Renderer) {
		self.cube_mesh = renderer.create_mesh(
			&wrath::Vertices::new(vec![
				//x    y  (z)
				1.0, 1.0, 0.0,
				0.0, 1.0, 0.0,
				0.0, 0.0, 0.0,
				1.0, 0.0, 0.0,
			]),
			&wrath::BufferLayout::new(&[
				wrath::BufferElement::Vec3
			]),
			&wrath::Indices::U8(vec![
				0, 1, 2,
				2, 3, 0,
			]),
		);
		self.flat_cube_shader = renderer.create_shader(
			Path::new("sandbox/assets/shaders/cube.glsl")
		);

		renderer.set_uniform(
			self.flat_cube_shader,
			"u_cols",
			COLS.into(), // same as wrath::ShaderUniform::U32(COLS)
		);
		renderer.set_uniform(
			self.flat_cube_shader,
			"u_rows",
			ROWS.into(),
		);

		renderer.set_clear_color((0.0, 0.0, 1.0).into());
	}
	fn on_render(&mut self, renderer: &mut dyn wrath::Renderer) {
		for x in 0..COLS {
			renderer.set_uniform(
				self.flat_cube_shader,
				"u_x",
				x.into(),
			);
			for y in 0..ROWS {
				renderer.set_uniform(
					self.flat_cube_shader,
					"u_y",
					y.into(),
				);
				if self.snake.contains(&(x, y)) {
					renderer.set_uniform(
						self.flat_cube_shader,
						"u_color",
						SNAKE_COLOR.into(),
					);
					renderer.render(self.cube_mesh, self.flat_cube_shader);
				} else if (x, y) == self.fruit {
					renderer.set_uniform(
						self.flat_cube_shader,
						"u_color",
						FRUIT_COLOR.into()
					);
					renderer.render(self.cube_mesh, self.flat_cube_shader);
				}
			}
		}
	}
}
