use crate::events::EventType;
use crate::LayerStack;
use crate::Window;
use crate::WindowProps;

pub struct Engine {
	window: Option<Box<dyn Window>>,
	is_running: bool,
	layer_stack: LayerStack,
}

impl Engine {
	pub fn new() -> Self {
		Self {
			window: None,
			is_running: true,
			layer_stack: LayerStack::new(),
		}
	}
	pub fn create_window(&mut self, props: WindowProps) {
		let win = crate::imp::gl::Window::new(props.title, props.size);

		self.window = Some(Box::new(win));
	}
	pub fn update(&mut self) {
		if let Some(window) = &mut self.window {
			for event in window.update() {
				if event.event_type() == EventType::WindowCloseRequested {
					self.is_running = false;
				}
				self.layer_stack.submit(event);
			}
		}
	}
	pub fn is_running(&self) -> bool {
		self.is_running
	}
	pub fn exit(&mut self) {
		self.is_running = false;
	}
	pub fn layer_stack(&mut self) -> &mut LayerStack {
		&mut self.layer_stack
	}
}
