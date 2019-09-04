#![feature(box_syntax)]

use std::time::Duration;

use wrath::Button;
use wrath::CallbackHandler;
use wrath::Engine;
use wrath::EngineProps;
use wrath::Float;
use wrath::Layer;
use wrath::LayerHandle;
use wrath::WindowProps;

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
		
	}
	fn on_exit(&mut self, engine: &mut Engine) {
		engine.layer_stack().remove_layer(self.ex_layer);
	}
}

struct ExampleLayer;

impl ExampleLayer {
	pub fn new() -> Self {
		Self
	}
}

impl Layer for ExampleLayer {
	fn on_update(&mut self, _dt: Duration) {
		// println!("dt: {}", dt.as_secs_f64());
	}
	fn on_window_resize(&mut self, size: (u32, u32)) {
		println!("Window resized: ({}, {})", size.0, size.1);
	}
	fn on_text_written(&mut self, which: char) -> bool {
		print!("{}", which);
		use std::io::Write;
		let _ = std::io::stdout().flush();
		false
	}
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
