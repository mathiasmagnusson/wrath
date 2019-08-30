pub struct Renderer {
	imp: Box<dyn RendererImpl>,
}

impl Renderer {
	pub fn new() -> Self {
		Self {
			imp: box crate::imp::opengl_renderer_impl::OpenGLRendererImpl::new(),
		}
	}
}

pub trait RendererImpl {
	fn set_clear_color(&mut self, r: u8, g: u8, b: u8);
	fn clear(&mut self);
	fn render(&mut self, /* stuff to render */);
}
