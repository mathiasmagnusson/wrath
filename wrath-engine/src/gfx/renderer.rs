pub struct Renderer {
	inner: Box<dyn PlatformRenderer>,
	clear_color: (u8, u8, u8),
}

impl Renderer {
	pub fn new() -> Self {
		let mut inner = box crate::platform::opengl_renderer::OpenGLRenderer::new();

		let clear_color = (0, 16, 32);

		inner.set_clear_color(clear_color);

		Self {
			inner,
			clear_color,
		}
	}
	// This function won't be
	// needed when the renderer
	// has it's own thread
	pub fn clear(&mut self) {
		self.inner.clear();
	}
	pub fn set_clear_color(&mut self, clear_color: (u8, u8, u8)) {
		if self.clear_color != clear_color {
			self.inner.set_clear_color(clear_color);
			self.clear_color = clear_color;
		}
	}
}

pub trait PlatformRenderer {
	fn clear(&mut self);
	fn set_clear_color(&mut self, clear_color: (u8, u8, u8));
	fn render(&mut self, /* stuff to render */);
}
