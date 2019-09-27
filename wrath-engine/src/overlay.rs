use crate::{Button, Renderer};

use whm::Float;

use std::time::Duration;

mod overlay_handle;
pub use overlay_handle::OverlayHandle;

mod overlay_stack;
pub use overlay_stack::OverlayStack;

#[allow(unused_variables)]
pub trait Overlay {
	fn on_attach(&mut self, renderer: &mut dyn Renderer) {}
	fn on_detach(&mut self, renderer: &mut dyn Renderer) {}
	fn on_update(&mut self, dt: Duration) {}
	fn on_render(&mut self, renderer: &mut dyn Renderer) {}
	fn on_window_close_requested(&mut self) {}
	fn on_window_resize(&mut self, size: (u32, u32)) {}
	fn on_key_press(&mut self, button: Button, repeat: bool) -> bool {
		false
	}
	fn on_key_release(&mut self, button: Button) -> bool {
		false
	}
	fn on_text_written(&mut self, which: char) -> bool {
		false
	}
	fn on_mouse_down(&mut self, button: Button) -> bool {
		false
	}
	fn on_mouse_up(&mut self, button: Button) -> bool {
		false
	}
	fn on_mouse_move(&mut self, position: (u32, u32), delta: (i32, i32)) -> bool {
		false
	}
	fn on_mouse_scroll(&mut self, delta: (Float, Float)) -> bool {
		false
	}
}
