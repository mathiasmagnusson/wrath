use crate::RendererImpl;

pub struct OpenGLRendererImpl;

impl OpenGLRendererImpl {
	pub fn new() -> Self {
		Self
	}
}

impl RendererImpl for OpenGLRendererImpl {
	fn set_clear_color(&mut self, r: u8, g: u8, b: u8) {

	}
	fn clear(&mut self) {

	}
	fn render(&mut self, /* stuff to render */) {

	}
}
