use crate::PlatformRenderer;

use wrath_math::Vec3;

pub struct OpenGLRenderer;

impl OpenGLRenderer {
	pub fn new() -> Self {
		Self
	}
}

impl PlatformRenderer for OpenGLRenderer {
	fn set_clear_color(&mut self, color: Vec3) {
		unsafe { gl::ClearColor(color.r(), color.g(), color.b(), 1.0) };
	}
	fn clear(&mut self) {
		unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
	}
	fn render(&mut self, /* stuff to render */) {

	}
}
