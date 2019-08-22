use crate::Window;
use crate::WindowProps;

pub struct Engine {
	window: Option<Box<dyn Window>>,
	is_running: bool,
}

impl Engine {
	pub fn new() -> Self {
		Self {
			window: None,
			is_running: true,
		}
	}
	pub fn create_window(&mut self, props: WindowProps) {
		let win = crate::imp::gl::Window::new(props.title, props.size);

		self.window = Some(Box::new(win));
	}
	pub fn update(&mut self) {
		if let Some(window) = &mut self.window {
			window.update();
			if window.close_requested() {
				self.is_running = false;
			}
		}
	}
	pub fn is_running(&self) -> bool {
		self.is_running
	}
	pub fn exit(&mut self) {
		self.is_running = false;
	}
}
