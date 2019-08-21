use crate::CallbackHandler;
use crate::Engine;

pub fn init<T>(mut handler: T) where T: CallbackHandler {
	let mut engine = Engine::new();
	handler.on_create(&mut engine);
	
	while engine.is_running() {
		engine.update();
	}

	handler.on_exit(&mut engine);
}
