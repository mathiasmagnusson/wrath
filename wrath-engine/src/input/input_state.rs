use super::Button;

pub struct InputState {
	mouse_position: (u32, u32),
	buttons: Button,
}

impl InputState {
	fn is_pressed(&self, button: Button) -> bool {
		self.buttons & button > 0
	}
}
