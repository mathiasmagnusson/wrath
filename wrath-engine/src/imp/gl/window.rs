use crate::button;
use crate::Button;
use crate::Event;
use crate::events::WindowCloseRequestedEvent;
use crate::events::WindowResizedEvent;
use crate::events::KeyPressedEvent;
use crate::events::KeyReleasedEvent;

pub struct Window {
	inner: glutin::Window,
	evt_loop: glutin::EventsLoop,
	title: String,
	close_requested: bool,
}

impl Window {
	pub fn new(title: String, size: (u32, u32)) -> Self {
		let el = glutin::EventsLoop::new();
		let win = glutin::Window::new(&el).unwrap();
		win.set_inner_size(glutin::dpi::LogicalSize::from(size));
		win.set_title(&title);

		Self {
			inner: win,
			evt_loop: el,
			title,
			close_requested: false,
		}
	}
}

impl crate::Window for Window {
	fn set_title(&mut self, title: String) {
		self.title = title;
	}
	fn get_title(&self) -> &str {
		&self.title
	}
	fn get_size(&self) -> (u32, u32) {
		self.inner.get_inner_size().unwrap().into()
	}
	fn update(&mut self) -> Vec<Box<dyn Event>> {
		let mut events: Vec<Box<dyn Event>> = vec![];

		self.evt_loop.poll_events(|event| match event {
			glutin::Event::WindowEvent { event, .. } => match event {
				glutin::WindowEvent::CloseRequested => {
					events.push(WindowCloseRequestedEvent::boxed())
				}
				glutin::WindowEvent::Resized(size) => {
					events.push(WindowResizedEvent::boxed(size.into()))
				}
				glutin::WindowEvent::KeyboardInput { input, ..} => {
					fn convert_key_event(input: glutin::KeyboardInput) -> Button {
						use button::*;
						let key = match input.virtual_keycode {
							Some(key) => key,
							None => return UNKNOWN,
						};
						match key {
							glutin::VirtualKeyCode::Space => SPACE,
							glutin::VirtualKeyCode::Apostrophe => APOSTROPHE,
							glutin::VirtualKeyCode::Comma => COMMA,
							glutin::VirtualKeyCode::Period => PERIOD,
							glutin::VirtualKeyCode::Slash => SLASH,
							glutin::VirtualKeyCode::Grave => TILDE,
							glutin::VirtualKeyCode::Key1 => NUM1,
							glutin::VirtualKeyCode::Key2 => NUM2,
							glutin::VirtualKeyCode::Key3 => NUM3,
							glutin::VirtualKeyCode::Key4 => NUM4,
							glutin::VirtualKeyCode::Key5 => NUM5,
							glutin::VirtualKeyCode::Key6 => NUM6,
							glutin::VirtualKeyCode::Key7 => NUM7,
							glutin::VirtualKeyCode::Key8 => NUM8,
							glutin::VirtualKeyCode::Key9 => NUM9,
							glutin::VirtualKeyCode::Key0 => NUM0,
							glutin::VirtualKeyCode::Subtract => MINUS,
							glutin::VirtualKeyCode::Equals => EQUALS,
							glutin::VirtualKeyCode::A => A,
							glutin::VirtualKeyCode::B => B,
							glutin::VirtualKeyCode::C => C,
							glutin::VirtualKeyCode::D => D,
							glutin::VirtualKeyCode::E => E,
							glutin::VirtualKeyCode::F => F,
							glutin::VirtualKeyCode::G => G,
							glutin::VirtualKeyCode::H => H,
							glutin::VirtualKeyCode::I => I,
							glutin::VirtualKeyCode::J => J,
							glutin::VirtualKeyCode::K => K,
							glutin::VirtualKeyCode::L => L,
							glutin::VirtualKeyCode::M => M,
							glutin::VirtualKeyCode::N => N,
							glutin::VirtualKeyCode::O => O,
							glutin::VirtualKeyCode::P => P,
							glutin::VirtualKeyCode::Q => Q,
							glutin::VirtualKeyCode::R => R,
							glutin::VirtualKeyCode::S => S,
							glutin::VirtualKeyCode::T => T,
							glutin::VirtualKeyCode::U => U,
							glutin::VirtualKeyCode::V => V,
							glutin::VirtualKeyCode::W => W,
							glutin::VirtualKeyCode::X => X,
							glutin::VirtualKeyCode::Y => Y,
							glutin::VirtualKeyCode::Z => Z,
							glutin::VirtualKeyCode::LBracket => L_BRACKET,
							glutin::VirtualKeyCode::RBracket => R_BRACKET,
							glutin::VirtualKeyCode::Backslash => BACKSLASH,
							glutin::VirtualKeyCode::Semicolon => SEMICOLON,
							glutin::VirtualKeyCode::Escape => ESC,
							glutin::VirtualKeyCode::Return => ENTER,
							glutin::VirtualKeyCode::Tab => TAB,
							glutin::VirtualKeyCode::Back => BACKSPACE,
							glutin::VirtualKeyCode::Insert => INSERT,
							glutin::VirtualKeyCode::Delete => DELETE,
							glutin::VirtualKeyCode::Right => ARROW_RIGHT,
							glutin::VirtualKeyCode::Left => ARROW_LEFT,
							glutin::VirtualKeyCode::Down => ARROW_DOWN,
							glutin::VirtualKeyCode::Up => ARROW_UP,
							glutin::VirtualKeyCode::PageUp => PG_UP,
							glutin::VirtualKeyCode::PageDown => PG_DOWN,
							glutin::VirtualKeyCode::Home => HOME,
							glutin::VirtualKeyCode::End => END,
							glutin::VirtualKeyCode::Capital => CAPS_LOCK,
							glutin::VirtualKeyCode::Scroll => SCROLL_LOCK,
							glutin::VirtualKeyCode::Numlock => NUMLOCK,
							glutin::VirtualKeyCode::Snapshot => PRINT_SCREEN,
							glutin::VirtualKeyCode::Pause => PAUSE,
							glutin::VirtualKeyCode::F1 => F1,
							glutin::VirtualKeyCode::F2 => F2,
							glutin::VirtualKeyCode::F3 => F3,
							glutin::VirtualKeyCode::F4 => F4,
							glutin::VirtualKeyCode::F5 => F5,
							glutin::VirtualKeyCode::F6 => F6,
							glutin::VirtualKeyCode::F7 => F7,
							glutin::VirtualKeyCode::F8 => F8,
							glutin::VirtualKeyCode::F9 => F9,
							glutin::VirtualKeyCode::F10 => F10,
							glutin::VirtualKeyCode::F11 => F11,
							glutin::VirtualKeyCode::F12 => F12,
							glutin::VirtualKeyCode::Numpad0 => NUMPAD0,
							glutin::VirtualKeyCode::Numpad1 => NUMPAD1,
							glutin::VirtualKeyCode::Numpad2 => NUMPAD2,
							glutin::VirtualKeyCode::Numpad3 => NUMPAD3,
							glutin::VirtualKeyCode::Numpad4 => NUMPAD4,
							glutin::VirtualKeyCode::Numpad5 => NUMPAD5,
							glutin::VirtualKeyCode::Numpad6 => NUMPAD6,
							glutin::VirtualKeyCode::Numpad7 => NUMPAD7,
							glutin::VirtualKeyCode::Numpad8 => NUMPAD8,
							glutin::VirtualKeyCode::Numpad9 => NUMPAD9,
							glutin::VirtualKeyCode::NumpadComma => NUMPAD_DEC,
							glutin::VirtualKeyCode::Divide => NUMPAD_DIV,
							glutin::VirtualKeyCode::Multiply => NUMPAD_MULT,
							glutin::VirtualKeyCode::Minus => NUMPAD_SUB,
							glutin::VirtualKeyCode::Add => NUMPAD_ADD,
							glutin::VirtualKeyCode::NumpadEnter => NUMPAD_ENTER,
							glutin::VirtualKeyCode::NumpadEquals => NUMPAD_EQ,
							glutin::VirtualKeyCode::LShift => L_SHIFT,
							glutin::VirtualKeyCode::LControl => L_CTRL,
							glutin::VirtualKeyCode::LAlt => L_ALT,
							glutin::VirtualKeyCode::LWin => L_SUPER,
							glutin::VirtualKeyCode::RShift => R_SHIFT,
							glutin::VirtualKeyCode::RControl => R_CTRL,
							glutin::VirtualKeyCode::RAlt => R_ALT,
							glutin::VirtualKeyCode::RWin => R_SUPER,
							_ => UNKNOWN,
						}
					}
					events.push(match input.state {
						glutin::ElementState::Pressed => { // check if it's already pressed to determine if its a repeat or not
							KeyPressedEvent::boxed(convert_key_event(input), false)
						}
						glutin::ElementState::Released => {
							KeyReleasedEvent::boxed(convert_key_event(input))
						}
					})
				}
				_ => {}
			},
			_ => {}
		});

		events
	}
	fn close_requested(&self) -> bool {
		self.close_requested
	}
}
