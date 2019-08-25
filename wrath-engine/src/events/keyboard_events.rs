use crate::Button;
use super::Event;
use super::EventType;
use crate::Layer;

pub struct KeyPressedEvent {
	is_handled: bool,
	button: Button,
	repeat: bool,
}

impl KeyPressedEvent {
	pub fn boxed(button: Button, repeat: bool) -> Box<Self> {
		box Self { is_handled: false, button, repeat }
	}
}

impl Event for KeyPressedEvent {
	fn is_handled(&self) -> bool {
		self.is_handled
	}
	fn dispatch(&mut self, layer: &mut dyn Layer) {
		self.is_handled = layer.on_key_press(self.button, self.repeat);
	}
	fn event_type(&self) -> EventType {
		EventType::KeyPressed
	}
}

pub struct KeyReleasedEvent {
	is_handled: bool,
	button: Button,
}

impl KeyReleasedEvent {
	pub fn boxed(button: Button) -> Box<Self> {
		box Self { is_handled: false, button }
	}
}

impl Event for KeyReleasedEvent {
	fn is_handled(&self) -> bool {
		self.is_handled
	}
	fn dispatch(&mut self, layer: &mut dyn Layer) {
		self.is_handled = layer.on_key_release(self.button);
	}
	fn event_type(&self) -> EventType {
		EventType::KeyReleased
	}
}

pub struct TextWrittenEvent {
	is_handled: bool,
	which: char,
}

impl TextWrittenEvent {
	pub fn boxed(which: char) -> Box<Self> {
		box Self { is_handled: false, which }
	}
}

impl Event for TextWrittenEvent {
	fn is_handled(&self) -> bool {
		self.is_handled
	}
	fn dispatch(&mut self, layer: &mut dyn Layer) {
		self.is_handled = layer.on_text_written(self.which);
	}
	fn event_type(&self) -> EventType {
		EventType::TextWritten
	}
}
