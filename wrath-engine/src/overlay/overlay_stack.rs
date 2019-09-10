use super::{Overlay, OverlayHandle};
use crate::{Event, Renderer};

use std::{collections::VecDeque, time::Duration};

pub struct OverlayStack {
	inner: VecDeque<(Box<dyn Overlay>, OverlayHandle)>,
	handle_counter: u32,
}

impl OverlayStack {
	pub fn new() -> Self {
		Self {
			inner: VecDeque::default(),
			handle_counter: 1,
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
	pub fn push_back(
		&mut self,
		mut overlay: Box<dyn Overlay>,
		renderer: &mut dyn Renderer,
	) -> OverlayHandle {
		overlay.on_attach(renderer);

		let handle = OverlayHandle::new(self.handle_counter);
		self.handle_counter += 1;
		self.inner.push_back((overlay, handle));
		handle
	}
	pub fn push_front(
		&mut self,
		mut overlay: Box<dyn Overlay>,
		renderer: &mut dyn Renderer,
	) -> OverlayHandle {
		overlay.on_attach(renderer);

		let handle = OverlayHandle::new(self.handle_counter);
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
