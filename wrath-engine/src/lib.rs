#![feature(box_syntax)]

mod callback_handler;
mod engine;
mod events;
mod init;
mod overlay;
mod platform;
mod rendering;
mod window;

pub mod input;

pub use callback_handler::CallbackHandler;
pub use engine::{Engine, EngineProps};
pub use events::Event;
pub use init::init;
pub use input::{Button, InputState};
pub use overlay::{Overlay, OverlayHandle, OverlayStack};
pub use rendering::{
	mesh::{BufferElement, BufferLayout, Indices, MeshHandle, Vertices},
	shader::{ShaderHandle, ShaderType, ShaderUniform},
	Renderer,
};
pub use window::{Window, WindowProps};

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
