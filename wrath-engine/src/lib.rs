#![feature(box_syntax)]

#[cfg(feature = "big-floats")]
pub type Float = f64;
#[cfg(not(feature = "big-floats"))]
pub type Float = f32;

mod callback_handler;
mod engine;
mod events;
mod imp;
mod init;
mod input;
mod layer;
mod window;


pub use callback_handler::CallbackHandler;
pub use engine::Engine;
pub use events::Event;
pub use init::init;
pub use input::button;
pub use input::Button;
pub use input::InputState;
pub use layer::Layer;
pub use layer::LayerHandle;
pub use layer::LayerStack;
pub use window::Window;
pub use window::WindowProps;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
