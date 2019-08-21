use crate::Engine;

pub trait CallbackHandler {
	fn on_create(&mut self, engine: &mut Engine);
	fn on_exit(&mut self, engine: &mut Engine);
}
