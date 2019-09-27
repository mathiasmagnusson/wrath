use whm::{
	Float,
	Vector3,
	Vector4,
};

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
	Vector3,
	Vector4,
}

impl BufferElement {
	pub fn count(&self) -> usize {
		match self {
			BufferElement::Vector3 => Vector3::len(),
			BufferElement::Vector4 => Vector4::len(),
		}
	}
	pub fn size(&self) -> usize {
		match self {
			BufferElement::Vector3 => std::mem::size_of::<Vector3>(),
			BufferElement::Vector4 => std::mem::size_of::<Vector4>(),
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
