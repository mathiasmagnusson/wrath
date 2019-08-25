use crate::CallbackHandler;
use crate::Engine;

pub fn init<T: CallbackHandler>(mut handler: T) {
	let mut engine = Engine::new();
	handler.on_create(&mut engine);

	while engine.is_running() {
		engine.update();
		handler.on_update(&mut engine);
	}

	handler.on_exit(&mut engine);
}
