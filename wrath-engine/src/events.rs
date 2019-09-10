use crate::Overlay;

mod application_events;
mod keyboard_events;
mod mouse_events;

pub use application_events::*;
pub use keyboard_events::*;
pub use mouse_events::*;

pub trait Event {
	fn is_handled(&self) -> bool;
	fn dispatch(&mut self, overlay: &mut dyn Overlay);
	fn event_type(&self) -> EventType;
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum EventType {
	WindowCloseRequested,
	WindowResized,
	KeyPressed,
	KeyReleased,
	TextWritten,
	MouseDown,
	MouseUp,
	MouseMove,
	MouseScrolled,
}
