use crate::Renderer;

use wrath_math::Vec3;

pub struct OpenGLRenderer {
	clear_color: Vec3,
}

impl OpenGLRenderer {
	pub fn new() -> Self {
		Self {
			clear_color: (0.0, 0.0, 0.0).into(),
		}
	}
}

impl Renderer for OpenGLRenderer {
	fn set_clear_color(&mut self, color: Vec3) {
		if color != self.clear_color {
			unsafe { gl::ClearColor(color.r(), color.g(), color.b(), 1.0) };
			self.clear_color = color;
		}
	}
	fn clear(&mut self) {
		unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
	}
}
