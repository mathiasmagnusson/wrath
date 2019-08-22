use wrath::Engine;
use wrath::CallbackHandler;

struct Application {

}

impl Application {
	fn new() -> Self {
		Application {}
	}
}

impl CallbackHandler for Application {
	fn on_create(&mut self, engine: &mut Engine) {
		println!("Hello");
		engine.create_window(wrath::WindowProps {
			title: "Wrath Engine Sandbox".into(),
			size: (1080, 720)
		});
	}
	fn on_exit(&mut self, _engine: &mut Engine) {
		println!("Bye");
	}
}

fn main() {
	wrath::init(Application::new());
}
