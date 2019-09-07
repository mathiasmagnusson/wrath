use wrath_math::Vec3;
use wrath_math::Float;

use std::fmt;
use std::path::Path;

pub trait Renderer {
	fn clear(&mut self);
	fn set_clear_color(&mut self, color: Vec3);
	fn create_shader(&mut self, path: &Path) -> ShaderHandle;
	fn delete_shader(&mut self, handle: ShaderHandle);
	fn set_uniform(&mut self, handle: ShaderHandle, name: &str, value: ShaderUniform);
	fn bind_shader(&mut self, handle: ShaderHandle);
}

// TODO: create "Bindable" trait so you can
// do renderer.bind(my_shit) with anything
// bindable

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderHandle {
	pub id: u32
}

impl ShaderHandle {
	pub fn new(id: u32) -> Self {
		Self {
			id
		}
	}
	pub fn none() -> Self {
		Self {
			id: 0
		}
	}
}

#[cfg(debug)]
impl Drop for ShaderHandle {
	fn drop(&mut self) {
		assert_eq!(self.id, 0, "ShaderHandle not properly deleted");
	}
}

pub enum ShaderType {
	Vertex,
	Fragment,
}

impl fmt::Display for ShaderType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			ShaderType::Vertex => "Vertex",
			ShaderType::Fragment => "Fragment",
		})
	}
}

pub enum ShaderUniform {
	Float(Float),
	Vec3(Vec3),
	Int(i32),
	Uint(u32),
}
