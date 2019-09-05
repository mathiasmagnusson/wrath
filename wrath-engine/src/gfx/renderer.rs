use wrath_math::Vec3;

pub struct Renderer {
	inner: Box<dyn PlatformRenderer>,
	clear_color: Vec3,
}

impl Renderer {
	pub fn new() -> Self {
		let mut inner = box crate::platform::opengl_renderer::OpenGLRenderer::new();

		let clear_color = (0.0, 0.06, 0.12).into();

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
	pub fn set_clear_color(&mut self, color: Vec3) {
		if color != self.clear_color {
			self.inner.set_clear_color(color);
			self.clear_color = color;
		}
	}
}

pub trait PlatformRenderer {
	fn clear(&mut self);
	fn set_clear_color(&mut self, clear_color: Vec3);
	fn render(&mut self, /* stuff to render */);
}
