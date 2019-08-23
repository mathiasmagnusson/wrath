use super::Button;

pub struct InputState {
	pub mouse_position: (u32, u32),
	pub buttons: Button,
}

impl InputState {
	const fn new() -> Self {
		Self {
			mouse_position: (0, 0),
			buttons: 0,
		}
	}
	pub fn mouse_position() -> (u32, u32) {
		unsafe { INPUT_STATE.mouse_position }
	}
	pub fn is_pressed(button: Button) -> bool {
		unsafe { INPUT_STATE.buttons & button > 0 }
	}
}

static mut INPUT_STATE: InputState = InputState::new();
