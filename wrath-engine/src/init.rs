use crate::CallbackHandler;
use crate::Engine;
use crate::EngineProps;

pub fn init<T: CallbackHandler>(mut handler: T, engine_props: EngineProps) {
	let mut engine = Engine::new(engine_props);
	handler.on_create(&mut engine);

	while engine.is_running() {
		engine.update();
		handler.on_update(&mut engine);
	}

	handler.on_exit(&mut engine);
}
