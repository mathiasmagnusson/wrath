#[cfg(feature = "big-floats")]
pub type Float = f64;
#[cfg(not(feature = "big-floats"))]
pub type Float = f32;

mod callback_handler;
mod engine;
mod events;
mod imp;
mod init;
mod window;

pub use callback_handler::CallbackHandler;
pub use engine::Engine;
pub use init::init;
pub use window::Window;
pub use window::WindowProps;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
