#![feature(box_syntax)]

use wrath::CallbackHandler;
use wrath::Engine;
use wrath::Layer;
use wrath::LayerHandle;

struct Application {
	ex_layer: LayerHandle,
}

impl Application {
	fn new() -> Self {
		Application {
			ex_layer: LayerHandle::none(),
		}
	}
}

impl CallbackHandler for Application {
	fn on_create(&mut self, engine: &mut Engine) {
		engine.create_window(wrath::WindowProps {
			title: "Wrath Engine Sandbox".into(),
			size: (1080, 720),
		});
		self.ex_layer = engine.layer_stack().push_front(box ExampleLayer::new());
	}
	fn on_exit(&mut self, engine: &mut Engine) {
		engine.layer_stack().remove_layer(self.ex_layer);
	}
}

struct ExampleLayer { }

impl ExampleLayer {
	pub fn new() -> Self {
		Self { }
	}
}

impl Layer for ExampleLayer {
	fn on_window_resize(&mut self, size: (u32, u32)) {
		println!("Window resized: ({}, {})", size.0, size.1);
	}
}

fn main() {
	wrath::init(Application::new());
}
