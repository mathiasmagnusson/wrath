use wrath_math::Vec3;

pub trait Renderer {
	fn clear(&mut self);
	fn set_clear_color(&mut self, color: Vec3);
}
