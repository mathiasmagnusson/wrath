use crate::PlatformRenderer;

pub struct OpenGLRenderer;

impl OpenGLRenderer {
	pub fn new() -> Self {
		Self
	}
}

impl PlatformRenderer for OpenGLRenderer {
	fn set_clear_color(&mut self, clear_color: (u8, u8, u8)) {
		let (r, g, b) = clear_color;
		unsafe { gl::ClearColor(r as f32 / 256.0, g as f32 / 256.0, b as f32 / 256.0, 1.0) };
	}
	fn clear(&mut self) {
		unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
	}
	fn render(&mut self, /* stuff to render */) {

	}
}
