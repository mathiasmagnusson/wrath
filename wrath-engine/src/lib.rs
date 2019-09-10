#![feature(box_syntax)]

mod callback_handler;
mod engine;
mod events;
mod rendering;
mod platform;
mod init;
mod overlay;
mod window;

pub mod input;

pub use callback_handler::CallbackHandler;
pub use engine::Engine;
pub use engine::EngineProps;
pub use events::Event;
pub use init::init;
pub use input::Button;
pub use input::InputState;
pub use overlay::Overlay;
pub use overlay::OverlayHandle;
pub use overlay::OverlayStack;
pub use rendering::BufferElement;
pub use rendering::BufferLayout;
pub use rendering::Indices;
pub use rendering::MeshHandle;
pub use rendering::Renderer;
pub use rendering::ShaderHandle;
pub use rendering::ShaderUniform;
pub use rendering::ShaderType;
pub use rendering::Vertices;
pub use window::Window;
pub use window::WindowProps;

pub use gl;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
