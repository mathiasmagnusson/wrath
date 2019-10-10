#![feature(box_syntax)]

// mod example_overlay;
// use example_overlay::ExampleOverlay;

mod snake_layer;
use snake_layer::SnakeOverlay;

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
		unsafe { gl::Viewport(0, 0, 800, 500) };

		self.ex_overlay = engine.push_overlay_front(box SnakeOverlay::new());
	}
	fn on_update(&mut self, _engine: &mut wrath::Engine) {
		// do shit
	}
	fn on_exit(&mut self, engine: &mut wrath::Engine) {
		engine.remove_overlay(self.ex_overlay);
	}
}
