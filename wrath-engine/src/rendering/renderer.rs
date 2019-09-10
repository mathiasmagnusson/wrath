use super::{
	mesh::{BufferLayout, Indices, MeshHandle, Vertices},
	shader::{ShaderHandle, ShaderUniform},
};

use wrath_math::Vec3;

use std::path::Path;

pub trait Renderer {
	fn clear(&mut self);
	fn set_clear_color(&mut self, color: Vec3);
	fn create_shader(&mut self, path: &Path) -> ShaderHandle;
	fn bind_shader(&mut self, handle: ShaderHandle);
	fn delete_shader(&mut self, handle: ShaderHandle);
	fn set_uniform(&mut self, handle: ShaderHandle, name: &str, value: ShaderUniform);
	fn create_mesh(
		&mut self,
		vertices: &Vertices,
		layout: &BufferLayout,
		indices: &Indices,
	) -> MeshHandle;
	fn bind_mesh(&mut self, handle: MeshHandle);
	fn delete_mesh(&mut self, handle: MeshHandle);
	fn render(&mut self, mh: MeshHandle, sh: ShaderHandle);
}
