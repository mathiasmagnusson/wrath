use wrath_math::Vec3;
use wrath_math::Vec4;
use wrath_math::Float;

use std::fmt;
use std::path::Path;

pub trait Renderer {
	fn clear(&mut self);
	fn set_clear_color(&mut self, color: Vec3);
	fn create_shader(&mut self, path: &Path) -> ShaderHandle;
	fn bind_shader(&mut self, handle: ShaderHandle);
	fn delete_shader(&mut self, handle: ShaderHandle);
	fn set_uniform(&mut self, handle: ShaderHandle, name: &str, value: ShaderUniform);
	fn create_mesh(&mut self, vertices: &Vertices, layout: &BufferLayout, indices: &Indices) -> MeshHandle;
	fn bind_mesh(&mut self, handle: MeshHandle);
	fn delete_mesh(&mut self, handle: MeshHandle);
	fn render(&mut self, mh: MeshHandle, sh: ShaderHandle);
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
	Vec4(Vec4),
	I32(i32),
	U32(u32),
}

impl Into<ShaderUniform> for Float {
	fn into(self) -> ShaderUniform {
		ShaderUniform::Float(self)
	}
}

impl Into<ShaderUniform> for Vec3 {
	fn into(self) -> ShaderUniform {
		ShaderUniform::Vec3(self)
	}
}

impl Into<ShaderUniform> for Vec4 {
	fn into(self) -> ShaderUniform {
		ShaderUniform::Vec4(self)
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

pub struct Vertices(Vec<Float>);

impl Vertices {
	pub fn new(v: Vec<Float>) -> Self {
		Self(v)
	}
	pub fn size(&self) -> usize {
		std::mem::size_of_val(self.0.as_slice())
	}
	pub fn as_ptr(&self) -> *const Float {
		self.0.as_ptr()
	}
}

impl Into<Vertices> for Vec<Float> {
	fn into(self) -> Vertices {
		Vertices(self)
	}
}

pub struct BufferLayout {
	pub types: Vec<BufferElement>,
	pub counts: Vec<usize>,
	pub offsets: Vec<usize>,
	pub stride: usize,
	pub len: usize,
}

impl BufferLayout {
	pub fn new(elements: &[BufferElement]) -> Self {
		let mut offset = 0;
		let mut types = Vec::with_capacity(elements.len());
		let mut counts = Vec::with_capacity(elements.len());
		let mut offsets = Vec::with_capacity(elements.len());
		for element in elements {
			types.push(*element);
			counts.push(element.count());
			offsets.push(offset);
			offset += element.size();
		}
		Self {
			types,
			counts,
			offsets,
			stride: offset,
			len: elements.len(),
		}
	}
}

#[derive(Clone, Copy)]
pub enum BufferElement {
	Vec3,
	Vec4,
}

impl BufferElement {
	pub fn count(&self) -> usize {
		match self {
			BufferElement::Vec3 => Vec3::len(),
			BufferElement::Vec4 => Vec4::len(),
		}
	}
	pub fn size(&self) -> usize {
		match self {
			BufferElement::Vec3 => std::mem::size_of::<Vec3>(),
			BufferElement::Vec4 => std::mem::size_of::<Vec4>(),
		}
	}
}

pub enum Indices {
	U8(Vec<u8>),
	U16(Vec<u16>),
	U32(Vec<u32>),
}

impl Indices {
	pub fn size(&self) -> usize {
		match self {
			Indices::U8 (v) => std::mem::size_of_val(v.as_slice()),
			Indices::U16(v) => std::mem::size_of_val(v.as_slice()),
			Indices::U32(v) => std::mem::size_of_val(v.as_slice()),
		}
	}
	pub fn as_ptr(&self) -> *const std::ffi::c_void {
		match self {
			Indices::U8 (v) => v.as_ptr() as *const _,
			Indices::U16(v) => v.as_ptr() as *const _,
			Indices::U32(v) => v.as_ptr() as *const _,
		}
	}
	pub fn len(&self) -> usize {
		match self {
			Indices::U8 (v) => v.len(),
			Indices::U16(v) => v.len(),
			Indices::U32(v) => v.len(),
		}
	}
}

impl Into<Indices> for Vec<u8> {
	fn into(self) -> Indices {
		Indices::U8(self)
	}
}

impl Into<Indices> for Vec<u16> {
	fn into(self) -> Indices {
		Indices::U16(self)
	}
}

impl Into<Indices> for Vec<u32> {
	fn into(self) -> Indices {
		Indices::U32(self)
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshHandle {
	pub id: u32
}

impl MeshHandle {
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
