mod application_events;
mod keyboard_events;

pub use application_events::*;
pub use keyboard_events::*;

use crate::Layer;

pub trait Event {
	fn is_handled(&self) -> bool;
	fn dispatch(&mut self, layer: &mut dyn Layer);
	fn event_type(&self) -> EventType;
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum EventType {
	WindowCloseRequested,
	WindowResized,
	KeyPressed,
	KeyReleased,
}
