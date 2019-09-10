use crate::Button;
use crate::Event;
use crate::Renderer;

use std::collections::VecDeque;
use std::ops::AddAssign;
use std::time::Duration;

use wrath_math::Float;

pub struct OverlayStack {
	inner: VecDeque<(Box<dyn Overlay>, OverlayHandle)>,
	handle_counter: OverlayHandle,
}

impl OverlayStack {
	pub fn new() -> Self {
		Self {
			inner: VecDeque::default(),
			handle_counter: OverlayHandle(1),
		}
	}
	pub fn submit(&mut self, mut event: Box<dyn Event>) {
		for overlay in self.inner.iter_mut() {
			event.dispatch(overlay.0.as_mut());
			if event.is_handled() {
				break;
			}
		}
	}
	pub fn call_update(&mut self, dt: Duration) {
		for overlay in self.inner.iter_mut() {
			overlay.0.on_update(dt);
		}
	}
	pub fn call_render(&mut self, renderer: &mut dyn Renderer) {
		for overlay in self.inner.iter_mut() {
			overlay.0.on_render(renderer);
		}
	}
	pub fn push_back(&mut self, mut overlay: Box<dyn Overlay>, renderer: &mut dyn Renderer) -> OverlayHandle {
		overlay.on_attach(renderer);

		let handle = self.handle_counter;
		self.handle_counter += 1;
		self.inner.push_back((overlay, handle));
		handle
	}
	pub fn push_front(&mut self, mut overlay: Box<dyn Overlay>, renderer: &mut dyn Renderer) -> OverlayHandle {
		overlay.on_attach(renderer);

		let handle = self.handle_counter;
		self.handle_counter += 1;
		self.inner.push_front((overlay, handle));
		handle
	}
	pub fn remove_overlay(&mut self, handle: OverlayHandle, renderer: &mut dyn Renderer) -> bool {
		for i in 0..self.inner.len() {
			if self.inner[i].1 == handle {
				if let Some((mut overlay, _handle)) = self.inner.remove(i) {
					overlay.on_detach(renderer);
				}
				return true;
			}
		}
		false
	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OverlayHandle(u32);

impl OverlayHandle {
	pub fn none() -> Self {
		OverlayHandle(0)
	}
}

impl AddAssign<u32> for OverlayHandle {
	fn add_assign(&mut self, rhs: u32) {
		self.0 += rhs;
	}
}

#[allow(unused_variables)]
pub trait Overlay {
	fn on_attach(&mut self, renderer: &mut dyn Renderer) {}
	fn on_detach(&mut self, renderer: &mut dyn Renderer) {}
	fn on_update(&mut self, dt: Duration) {}
	fn on_render(&mut self, renderer: &mut dyn Renderer) {}
	fn on_window_close_requested(&mut self) {}
	fn on_window_resize(&mut self, size: (u32, u32)) {}
	fn on_key_press(&mut self, button: Button, repeat: bool) -> bool { false }
	fn on_key_release(&mut self, button: Button) -> bool { false }
	fn on_text_written(&mut self, which: char) -> bool { false }
	fn on_mouse_down(&mut self, button: Button) -> bool { false }
	fn on_mouse_up(&mut self, button: Button) -> bool { false }
	fn on_mouse_move(&mut self, position: (u32, u32), delta: (i32, i32)) -> bool { false }
	fn on_mouse_scroll(&mut self, delta: (Float, Float)) -> bool { false }
}
