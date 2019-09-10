use super::Event;
use super::EventType;
use crate::Overlay;

pub struct WindowCloseRequestedEvent;

impl WindowCloseRequestedEvent {
	pub fn boxed() -> Box<Self> {
		box WindowCloseRequestedEvent
	}
}

impl Event for WindowCloseRequestedEvent {
	fn is_handled(&self) -> bool {
		false
	}
	fn dispatch(&mut self, overlay: &mut dyn Overlay) {
		overlay.on_window_close_requested();
	}
	fn event_type(&self) -> EventType {
		EventType::WindowCloseRequested
	}
}

pub struct WindowResizedEvent(u32, u32);

impl WindowResizedEvent {
	pub fn boxed(size: (u32, u32)) -> Box<Self> {
		box WindowResizedEvent(size.0, size.1)
	}
}

impl Event for WindowResizedEvent {
	fn is_handled(&self) -> bool {
		false
	}
	fn dispatch(&mut self, overlay: &mut dyn Overlay) {
		overlay.on_window_resize((self.0, self.1));
	}
	fn event_type(&self) -> EventType {
		EventType::WindowResized
	}
}
