use super::{Event, EventType};
use crate::{Button, Overlay};

use whmath::Float;

pub struct MouseDownEvent {
	is_handled: bool,
	button: Button,
}

impl MouseDownEvent {
	pub fn boxed(button: Button) -> Box<Self> {
		box Self {
			is_handled: false,
			button,
		}
	}
}

impl Event for MouseDownEvent {
	fn is_handled(&self) -> bool {
		self.is_handled
	}
	fn dispatch(&mut self, overlay: &mut dyn Overlay) {
		self.is_handled = overlay.on_mouse_down(self.button);
	}
	fn event_type(&self) -> EventType {
		EventType::MouseDown
	}
}

pub struct MouseUpEvent {
	is_handled: bool,
	button: Button,
}

impl MouseUpEvent {
	pub fn boxed(button: Button) -> Box<Self> {
		box Self {
			is_handled: false,
			button,
		}
	}
}

impl Event for MouseUpEvent {
	fn is_handled(&self) -> bool {
		self.is_handled
	}
	fn dispatch(&mut self, overlay: &mut dyn Overlay) {
		self.is_handled = overlay.on_mouse_up(self.button);
	}
	fn event_type(&self) -> EventType {
		EventType::MouseUp
	}
}

pub struct MouseMoveEvent {
	is_handled: bool,
	position: (u32, u32),
	delta: (i32, i32),
}

impl MouseMoveEvent {
	pub fn boxed(position: (u32, u32), delta: (i32, i32)) -> Box<Self> {
		box Self {
			is_handled: false,
			position,
			delta,
		}
	}
}

impl Event for MouseMoveEvent {
	fn is_handled(&self) -> bool {
		self.is_handled
	}
	fn dispatch(&mut self, overlay: &mut dyn Overlay) {
		self.is_handled = overlay.on_mouse_move(self.position, self.delta);
	}
	fn event_type(&self) -> EventType {
		EventType::MouseMove
	}
}

pub struct MouseScrolledEvent {
	is_handled: bool,
	delta: (Float, Float),
}

impl MouseScrolledEvent {
	pub fn boxed(delta: (Float, Float)) -> Box<Self> {
		box Self {
			is_handled: false,
			delta,
		}
	}
}

impl Event for MouseScrolledEvent {
	fn is_handled(&self) -> bool {
		self.is_handled
	}
	fn dispatch(&mut self, overlay: &mut dyn Overlay) {
		self.is_handled = overlay.on_mouse_scroll(self.delta);
	}
	fn event_type(&self) -> EventType {
		EventType::MouseScrolled
	}
}
