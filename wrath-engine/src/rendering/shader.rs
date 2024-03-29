use whm::{
	Float,
	Vector3,
	Vector4,
};

use std::fmt;

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
	Vector3(Vector3),
	Vector4(Vector4),
	I32(i32),
	U32(u32),
}

impl Into<ShaderUniform> for Float {
	fn into(self) -> ShaderUniform {
		ShaderUniform::Float(self)
	}
}

impl Into<ShaderUniform> for Vector3 {
	fn into(self) -> ShaderUniform {
		ShaderUniform::Vector3(self)
	}
}

impl Into<ShaderUniform> for Vector4 {
	fn into(self) -> ShaderUniform {
		ShaderUniform::Vector4(self)
	}
}

impl Into<ShaderUniform> for i32 {
	fn into(self) -> ShaderUniform {
		ShaderUniform::I32(self)
	}
}

impl Into<ShaderUniform> for u32 {
	fn into(self) -> ShaderUniform {
		ShaderUniform::U32(self)
	}
}
