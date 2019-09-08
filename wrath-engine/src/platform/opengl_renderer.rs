use crate::Renderer;
use crate::ShaderHandle;
use crate::ShaderUniform;
use crate::ShaderType;

use wrath_math::Vec3;

use std::collections::HashMap;
use std::ffi::CString;

use std::path::Path;
use std::fs;

pub struct OpenGLRenderer {
	clear_color: Vec3,
	shaders: HashMap<ShaderHandle, Shader>,
	bound_shader: ShaderHandle,
	shader_handle_counter: u32,
}

impl OpenGLRenderer {
	pub fn new() -> Self {
		Self {
			clear_color: (0.0, 0.0, 0.0).into(),
			shaders: Default::default(),
			bound_shader: ShaderHandle::none(),
			shader_handle_counter: 1,
		}
	}
}

impl Drop for OpenGLRenderer {
	fn drop(&mut self) {
		for shader in self.shaders.values() {
			unsafe { gl::DeleteProgram(shader.id) };
		}
	}
}

impl Renderer for OpenGLRenderer {
	fn set_clear_color(&mut self, color: Vec3) {
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

		let handle = ShaderHandle::new(self.shader_handle_counter);
		self.shader_handle_counter += 1;

		self.shaders.insert(handle, shader);

		handle
	}
	fn delete_shader(&mut self, handle: ShaderHandle) {
		let shader = self.shaders.remove(&handle)
			.expect("Unknown shader");
		unsafe {
			gl::DeleteProgram(shader.id);
		}
	}
	fn bind_shader(&mut self, handle: ShaderHandle) {
		if handle == self.bound_shader { return };
		let shader = &self.shaders[&handle];
		unsafe {
			gl::UseProgram(shader.id);
		}
		self.bound_shader = handle;
	}
	fn set_uniform(&mut self, handle: ShaderHandle, name: &str, val: ShaderUniform) {
		unsafe {
			let shader = self.shaders.get_mut(&handle)
				.expect("Unknown shader");

			if !shader.uniform_cache.contains_key(name) {
				shader.uniform_cache.insert(
					name.into(),
					gl::GetUniformLocation(
						shader.id,
						CString::new(name)
							.unwrap()
							.as_ptr()
					)
				);
			}
			let location = shader.uniform_cache[name];

			self.bind_shader(handle);

			match val {
				ShaderUniform::Float(val) => gl::Uniform1f(location, val),
				ShaderUniform::Vec3(val) => gl::Uniform3f(location, val.x(), val.y(), val.z()),
				ShaderUniform::Int(val) => gl::Uniform1i(location, val),
				ShaderUniform::Uint(val) => gl::Uniform1ui(location, val),
			}
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
		let id = gl::CreateShader(match type_ {
			ShaderType::Vertex => gl::VERTEX_SHADER,
			ShaderType::Fragment => gl::FRAGMENT_SHADER,
		});
		let ptr = src.as_ptr() as *const i8;
		let len = src.len() as i32;
		gl::ShaderSource(id, 1, &ptr, &len);
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
			gl::AttachShader(id, *shader);
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
		let source = fs::read_to_string(path)
			.expect(&format!("Error reading shader source file at {}", path.display()));

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
