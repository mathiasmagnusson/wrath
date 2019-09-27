use crate::{
	BufferLayout, Indices, MeshHandle, Renderer, ShaderHandle, ShaderType, ShaderUniform, Vertices,
};

use whm::Vector3;

use std::collections::HashMap;
use std::ffi::CString;
use std::fs;
use std::path::Path;

pub struct OpenGLRenderer {
	clear_color: Vector3,
	handle_counter: u32,
	shaders: HashMap<ShaderHandle, Shader>,
	bound_shader: ShaderHandle,
	meshes: HashMap<MeshHandle, Mesh>,
	bound_mesh: MeshHandle,
}

impl OpenGLRenderer {
	pub fn new() -> Self {
		unsafe {
			gl::Enable(gl::BLEND);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		}
		Self {
			clear_color: (0.0, 0.0, 0.0).into(),
			handle_counter: 1,
			shaders: Default::default(),
			bound_shader: ShaderHandle::none(),
			meshes: Default::default(),
			bound_mesh: MeshHandle::none(),
		}
	}
	fn _delete_shader(&mut self, shader: Shader) {
		unsafe {
			gl_call("glDeleteProgram", || {
				gl::DeleteProgram(shader.id);
			});
		}
	}
	fn _delete_mesh(&mut self, mesh: Mesh) {
		unsafe {
			gl::DeleteVertexArrays(1, &mesh.va);
			gl::DeleteBuffers(1, &mesh.vb);
			gl::DeleteBuffers(1, &mesh.ib);
		}
	}
}

impl Drop for OpenGLRenderer {
	fn drop(&mut self) {
		let shaders = self
			.shaders
			.drain()
			.map(|(_, s)| s)
			.collect::<Vec<Shader>>();
		for shader in shaders.into_iter() {
			self._delete_shader(shader);
		}
		let meshes = self.meshes.drain().map(|(_, m)| m).collect::<Vec<Mesh>>();
		for mesh in meshes {
			self._delete_mesh(mesh);
		}
	}
}

impl Renderer for OpenGLRenderer {
	fn set_clear_color(&mut self, color: Vector3) {
		if color != self.clear_color {
			unsafe { gl::ClearColor(color.r(), color.g(), color.b(), 1.0) };
			self.clear_color = color;
		}
	}
	fn clear(&mut self) {
		unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
	}
	fn create_shader(&mut self, path: &Path) -> ShaderHandle {
		let (vertex, fragment) = read_shader_source(path);
		let vertex = compile_shader(&vertex, ShaderType::Vertex);
		let fragment = compile_shader(&fragment, ShaderType::Fragment);

		let shader = link_shaders(&[vertex, fragment]);

		let handle = ShaderHandle::new(self.handle_counter);
		self.handle_counter += 1;

		self.shaders.insert(handle, shader);

		handle
	}
	fn bind_shader(&mut self, handle: ShaderHandle) {
		if handle == self.bound_shader {
			return;
		};
		let shader = &self.shaders[&handle];
		unsafe {
			gl::UseProgram(shader.id);
		}
		self.bound_shader = handle;
	}
	fn delete_shader(&mut self, handle: ShaderHandle) {
		let shader = self.shaders.remove(&handle).expect("Unknown shader");
		self._delete_shader(shader);
	}
	fn set_uniform(&mut self, handle: ShaderHandle, name: &str, val: ShaderUniform) {
		unsafe {
			let shader = self.shaders.get_mut(&handle).expect("Unknown shader");

			if !shader.uniform_cache.contains_key(name) {
				shader.uniform_cache.insert(
					name.into(),
					gl_call("getGetUniformLocation", || {
						gl::GetUniformLocation(shader.id, CString::new(name).unwrap().as_ptr())
					}),
				);
			}
			let location = shader.uniform_cache[name];

			self.bind_shader(handle);

			gl_call("glUniform*", || match val {
				ShaderUniform::Float(val) => gl::Uniform1f(location, val),
				ShaderUniform::Vector3(val) => gl::Uniform3f(location, val[0], val[1], val[2]),
				ShaderUniform::Vector4(val) => gl::Uniform4f(location, val[0], val[1], val[2], val[3]),
				ShaderUniform::I32(val) => gl::Uniform1i(location, val),
				ShaderUniform::U32(val) => gl::Uniform1ui(location, val),
			});
		}
	}
	fn create_mesh(
		&mut self,
		vertices: &Vertices,
		layout: &BufferLayout,
		indices: &Indices,
	) -> MeshHandle {
		let mesh = Mesh::new(vertices, layout, indices);

		let handle = MeshHandle::new(self.handle_counter);
		self.handle_counter += 1;
		self.meshes.insert(handle, mesh);

		handle
	}
	fn bind_mesh(&mut self, handle: MeshHandle) {
		if handle == self.bound_mesh {
			return;
		};
		let mesh = &self.meshes[&handle];
		unsafe {
			gl_call("bind mesh", || {
				gl::BindVertexArray(mesh.va);
				gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vb);
				gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ib);
			});
		}
		self.bound_mesh = handle;
	}
	fn delete_mesh(&mut self, handle: MeshHandle) {
		let mesh = self
			.meshes
			.remove(&handle)
			.expect("Tried to delete unknown mesh");
		self._delete_mesh(mesh);
	}
	fn render(&mut self, mh: MeshHandle, sh: ShaderHandle) {
		self.bind_mesh(mh);
		self.bind_shader(sh);
		let mesh = &self.meshes[&mh];
		unsafe {
			gl_call("glDrawElements", || {
				gl::DrawElements(
					gl::TRIANGLES,
					mesh.index_count,
					mesh.index_type,
					std::ptr::null_mut(),
				);
			});
		}
	}
}

type PartialShader = u32;

struct Shader {
	pub id: u32,
	pub uniform_cache: HashMap<String, i32>,
}

impl Shader {
	pub fn new(id: u32) -> Self {
		Self {
			id,
			uniform_cache: Default::default(),
		}
	}
}

fn compile_shader(src: &str, type_: ShaderType) -> PartialShader {
	unsafe {
		let id = gl_call("glCreateShader", || {
			gl::CreateShader(match type_ {
				ShaderType::Vertex => gl::VERTEX_SHADER,
				ShaderType::Fragment => gl::FRAGMENT_SHADER,
			})
		});
		let ptr = src.as_ptr() as *const i8;
		let len = src.len() as i32;
		gl_call("glShaderSource", || gl::ShaderSource(id, 1, &ptr, &len));
		gl::CompileShader(id);

		let mut success = 0;
		gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
		if success == 0 {
			println!("\x1b[31m{} Shader Compilation Failiure:\x1b[0m", type_);

			let mut len = 0;
			gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);

			let mut log = vec![b'a'; len as usize - 1];
			gl::GetShaderInfoLog(id, len, &mut len, log.as_mut_ptr() as *mut i8);

			let log = String::from_utf8_lossy(&log);

			println!("{}", log);
			panic!();
		}

		id
	}
}

fn link_shaders(shaders: &[PartialShader]) -> Shader {
	unsafe {
		let id = gl::CreateProgram();

		for shader in shaders {
			gl_call("glAttachShader", || {
				gl::AttachShader(id, *shader);
			});
		}
		gl::LinkProgram(id);

		let mut success = 0;
		gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
		if success == 0 {
			println!("\x1b[31mShader Program Linking Failiure:\x1b[0m");

			let mut len = 0;
			gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);

			let mut log = Vec::<u8>::with_capacity(len as usize);
			gl::GetProgramInfoLog(id, len, &mut len, log.as_mut_ptr() as *mut i8);

			let log = String::from_utf8_lossy(&log);

			println!("{}", log);
			panic!();
		}

		for shader in shaders {
			gl::DetachShader(id, *shader);
		}

		Shader::new(id)
	}
}

// TODO: maybe make magenta shaders if something fails to load
fn read_shader_source(path: &Path) -> (String, String) {
	if path.is_dir() {
		let vertex = path.join("vertex.glsl");
		let fragment = path.join("fragment.glsl");
		assert!(
			vertex.is_file(),
			"Could not find vertex.glsl in specified directory {}",
			path.display(),
		);
		assert!(
			fragment.is_file(),
			"Could not find fragment.glsl in specified directory {}",
			path.display(),
		);
		let vertex = fs::read_to_string(vertex).unwrap();
		let fragment = fs::read_to_string(fragment).unwrap();

		(vertex, fragment)
	} else {
		let source = fs::read_to_string(path).expect(&format!(
			"Error reading shader source file at {}",
			path.display()
		));

		let mut vertex = String::new();
		let mut fragment = String::new();

		let mut dst = None;
		for line in source.lines() {
			if line.starts_with("#type") {
				if line.starts_with("#type vertex") {
					dst = Some(true);
				} else if line.starts_with("#type fragment") {
					dst = Some(false);
				} else {
					panic!("Unknown shader type {}", &line[5..]);
				}
			} else if let Some(dst) = dst {
				if dst {
					vertex.push_str(line);
					vertex.push('\n');
				} else {
					fragment.push_str(line);
					fragment.push('\n');
				}
			}
		}

		(vertex, fragment)
	}
}

struct Mesh {
	va: u32,
	vb: u32,
	ib: u32,
	index_count: i32,
	index_type: u32,
}

impl Mesh {
	pub fn new(vertices: &Vertices, layout: &BufferLayout, indices: &Indices) -> Self {
		unsafe {
			let mut va = 0;
			gl::CreateVertexArrays(1, &mut va);
			gl::BindVertexArray(va);

			let mut vb = 0;
			gl::CreateBuffers(1, &mut vb);
			gl::BindBuffer(gl::ARRAY_BUFFER, vb);

			gl_call("glBufferData array buffer", || {
				gl::BufferData(
					gl::ARRAY_BUFFER,
					vertices.size() as isize,
					vertices.as_ptr() as *const _,
					gl::STATIC_DRAW,
				);
			});

			for i in 0..layout.len {
				gl_call("glEnableVertexAttribArray | glVertexAttribPointer", || {
					gl::EnableVertexAttribArray(i as u32);
					gl::VertexAttribPointer(
						i as u32,
						layout.counts[i] as i32,
						gl::FLOAT, // data type
						gl::FALSE, // normalize
						layout.stride as i32,
						layout.offsets[i] as _,
					);
				});
			}

			let mut ib = 0;
			gl::CreateBuffers(1, &mut ib);
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ib);

			gl_call("glBufferData index buffer", || {
				gl::BufferData(
					gl::ELEMENT_ARRAY_BUFFER,
					indices.size() as isize,
					indices.as_ptr(),
					gl::STATIC_DRAW,
				);
			});

			Self {
				va,
				vb,
				ib,
				index_count: indices.len() as i32,
				index_type: match indices {
					Indices::U8(_) => gl::UNSIGNED_BYTE,
					Indices::U16(_) => gl::UNSIGNED_SHORT,
					Indices::U32(_) => gl::UNSIGNED_INT,
				},
			}
		}
	}
}

fn gl_call<T, F: FnOnce() -> T>(ident: &'static str, f: F) -> T {
	if cfg!(debug_assertions) {
		unsafe {
			loop {
				if gl::GetError() == gl::NO_ERROR {
					break;
				};
			}

			let ret = f();

			let err = gl::GetError();
			if err != gl::NO_ERROR {
				panic!(
					"Open gl error at {}: {}",
					ident,
					match err {
						gl::INVALID_ENUM => "INVALID_ENUM",
						gl::INVALID_VALUE => "INVALID_VALUE",
						gl::INVALID_OPERATION => "INVALID_OPERATION",
						gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
						gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
						gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
						gl::STACK_OVERFLOW => "STACK_OVERFLOW",
						_ => "undefined",
					}
				);
			}
			ret
		}
	} else {
		f()
	}
}
