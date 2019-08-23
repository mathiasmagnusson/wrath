use crate::events::KeyPressedEvent;
use crate::events::KeyReleasedEvent;
use crate::events::MouseDownEvent;
use crate::events::MouseMoveEvent;
use crate::events::MouseScrolledEvent;
use crate::events::MouseUpEvent;
use crate::events::TextWrittenEvent;
use crate::events::WindowCloseRequestedEvent;
use crate::events::WindowResizedEvent;
use crate::input::get_mouse_position;
use crate::Button;
use crate::Event;
use crate::Float;

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
				glutin::WindowEvent::KeyboardInput { input, .. } => {
					fn convert_key_event(input: glutin::KeyboardInput) -> Button {
						use Button::*;
						let key = match input.virtual_keycode {
							Some(key) => key,
							None => return Unknown,
						};
						match key {
							glutin::VirtualKeyCode::Space => Space,
							glutin::VirtualKeyCode::Apostrophe => Apostrophe,
							glutin::VirtualKeyCode::Comma => Comma,
							glutin::VirtualKeyCode::Period => Period,
							glutin::VirtualKeyCode::Slash => Slash,
							glutin::VirtualKeyCode::Grave => Tilde,
							glutin::VirtualKeyCode::Key1 => Num1,
							glutin::VirtualKeyCode::Key2 => Num2,
							glutin::VirtualKeyCode::Key3 => Num3,
							glutin::VirtualKeyCode::Key4 => Num4,
							glutin::VirtualKeyCode::Key5 => Num5,
							glutin::VirtualKeyCode::Key6 => Num6,
							glutin::VirtualKeyCode::Key7 => Num7,
							glutin::VirtualKeyCode::Key8 => Num8,
							glutin::VirtualKeyCode::Key9 => Num9,
							glutin::VirtualKeyCode::Key0 => Num0,
							glutin::VirtualKeyCode::Subtract => Minus,
							glutin::VirtualKeyCode::Equals => Equals,
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
							glutin::VirtualKeyCode::LBracket => BracketLeft,
							glutin::VirtualKeyCode::RBracket => BracketRight,
							glutin::VirtualKeyCode::Backslash => Backslash,
							glutin::VirtualKeyCode::Semicolon => Semicolon,
							glutin::VirtualKeyCode::Escape => Esc,
							glutin::VirtualKeyCode::Return => Enter,
							glutin::VirtualKeyCode::Tab => Tab,
							glutin::VirtualKeyCode::Back => Backspace,
							glutin::VirtualKeyCode::Insert => Insert,
							glutin::VirtualKeyCode::Delete => Delete,
							glutin::VirtualKeyCode::Right => ArrowRight,
							glutin::VirtualKeyCode::Left => ArrowLeft,
							glutin::VirtualKeyCode::Down => ArrowDown,
							glutin::VirtualKeyCode::Up => ArrowUp,
							glutin::VirtualKeyCode::PageUp => PgUp,
							glutin::VirtualKeyCode::PageDown => PgDown,
							glutin::VirtualKeyCode::Home => Home,
							glutin::VirtualKeyCode::End => End,
							glutin::VirtualKeyCode::Capital => CapsLock,
							glutin::VirtualKeyCode::Scroll => ScrollLock,
							glutin::VirtualKeyCode::Numlock => NumLock,
							glutin::VirtualKeyCode::Snapshot => PrintScreen,
							glutin::VirtualKeyCode::Pause => Pause,
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
							glutin::VirtualKeyCode::Numpad0 => NumPad0,
							glutin::VirtualKeyCode::Numpad1 => NumPad1,
							glutin::VirtualKeyCode::Numpad2 => NumPad2,
							glutin::VirtualKeyCode::Numpad3 => NumPad3,
							glutin::VirtualKeyCode::Numpad4 => NumPad4,
							glutin::VirtualKeyCode::Numpad5 => NumPad5,
							glutin::VirtualKeyCode::Numpad6 => NumPad6,
							glutin::VirtualKeyCode::Numpad7 => NumPad7,
							glutin::VirtualKeyCode::Numpad8 => NumPad8,
							glutin::VirtualKeyCode::Numpad9 => NumPad9,
							glutin::VirtualKeyCode::NumpadComma => NumPadDec,
							glutin::VirtualKeyCode::Divide => NumPadDiv,
							glutin::VirtualKeyCode::Multiply => NumPadMult,
							glutin::VirtualKeyCode::Minus => NumPadSub,
							glutin::VirtualKeyCode::Add => NumPadAdd,
							glutin::VirtualKeyCode::NumpadEnter => NumPadEnter,
							glutin::VirtualKeyCode::NumpadEquals => NumPadEq,
							glutin::VirtualKeyCode::LShift => LShift,
							glutin::VirtualKeyCode::LControl => LCtrl,
							glutin::VirtualKeyCode::LAlt => LAlt,
							glutin::VirtualKeyCode::LWin => LSuper,
							glutin::VirtualKeyCode::RShift => RShift,
							glutin::VirtualKeyCode::RControl => RCtrl,
							glutin::VirtualKeyCode::RAlt => RAlt,
							glutin::VirtualKeyCode::RWin => RSuper,
							_ => Unknown,
						}
					}
					events.push(match input.state {
						glutin::ElementState::Pressed => {
							let button = convert_key_event(input);
							KeyPressedEvent::boxed(button, button.is_pressed())
						}
						glutin::ElementState::Released => {
							KeyReleasedEvent::boxed(convert_key_event(input))
						}
					})
				}
				glutin::WindowEvent::ReceivedCharacter(which) => {
					events.push(TextWrittenEvent::boxed(which));
				}
				glutin::WindowEvent::MouseInput { state, button, .. } => match state {
					glutin::ElementState::Pressed => {
						events.push(MouseDownEvent::boxed(match button {
							glutin::MouseButton::Left => Button::MouseLeft,
							glutin::MouseButton::Middle => Button::MouseMiddle,
							glutin::MouseButton::Right => Button::MouseRight,
							glutin::MouseButton::Other(8) => Button::Mouse4,
							glutin::MouseButton::Other(9) => Button::Mouse5,
							// glutin::MouseButton::Other(2) => Button::Mouse6,
							// glutin::MouseButton::Other(3) => Button::Mouse7,
							// glutin::MouseButton::Other(4) => Button::Mouse8,
							// TODO: are there more?
							glutin::MouseButton::Other(_) => Button::Unknown,
						}));
					}
					glutin::ElementState::Released => {
						events.push(MouseUpEvent::boxed(match button {
							glutin::MouseButton::Left => Button::MouseLeft,
							glutin::MouseButton::Middle => Button::MouseMiddle,
							glutin::MouseButton::Right => Button::MouseRight,
							glutin::MouseButton::Other(8) => Button::Mouse4,
							glutin::MouseButton::Other(9) => Button::Mouse5,
							// glutin::MouseButton::Other(?) => Button::Mouse6,
							// glutin::MouseButton::Other(?) => Button::Mouse7,
							// glutin::MouseButton::Other(?) => Button::Mouse8,
							// TODO: are there more?
							glutin::MouseButton::Other(_) => Button::Unknown,
						}));
					}
				},
				glutin::WindowEvent::CursorMoved { position, .. } => {
					let pos: (u32, u32) = (position.x as u32, position.y as u32);
					let old_pos = get_mouse_position();
					let delta: (i32, i32) = (
						pos.0 as i32 - old_pos.0 as i32,
						pos.1 as i32 - old_pos.1 as i32,
					);
					events.push(MouseMoveEvent::boxed(pos, delta));
				}
				glutin::WindowEvent::MouseWheel { delta, .. } => {
					events.push(MouseScrolledEvent::boxed(match delta {
						// TODO: maybe use some multiplier here m8
						glutin::MouseScrollDelta::LineDelta(x, y) => (x.into(), y.into()),
						glutin::MouseScrollDelta::PixelDelta(d) => (d.x as Float, d.y as Float),
					}))
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
