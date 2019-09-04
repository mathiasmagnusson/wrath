#![feature(box_syntax)]

#[cfg(feature = "big-floats")]
pub type Float = f64;
#[cfg(not(feature = "big-floats"))]
pub type Float = f32;

mod callback_handler;
mod engine;
mod events;
mod gfx;
mod imp;
mod init;
mod layer;
mod window;

pub mod input;

pub use callback_handler::CallbackHandler;
pub use engine::Engine;
pub use engine::EngineProps;
pub use events::Event;
pub use init::init;
pub use input::Button;
pub use input::InputState;
pub use layer::Layer;
pub use layer::LayerHandle;
pub use layer::LayerStack;
pub use gfx::Renderer;
pub use gfx::RendererImpl;
pub use window::Window;
pub use window::WindowProps;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
