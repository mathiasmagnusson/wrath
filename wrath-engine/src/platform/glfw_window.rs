use crate::events::*;
use crate::input::get_mouse_position;
use crate::Button;
use crate::Event;
use crate::Window;
use crate::WindowProps;

use std::sync::Once;

use wrath_math::Float;

type GlfwEventLoop = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;
static mut GLFW_HANDLE: Option<glfw::Glfw> = None;
static INIT_GLFW: Once = Once::new();
fn glfw_handle() -> glfw::Glfw {
	unsafe {
		INIT_GLFW.call_once(|| {
			GLFW_HANDLE = Some(glfw::init(glfw::FAIL_ON_ERRORS).unwrap());
		});
		GLFW_HANDLE.unwrap()
	}
}

pub struct GLFWWindow {
	inner: glfw::Window,
	evt_recv: GlfwEventLoop,
	title: String,
	close_requested: bool,
}

impl GLFWWindow {
	pub fn new(props: WindowProps) -> Self {
		// TODO: save this globally if needed

		let (mut window, evt_recv) = glfw_handle().create_window(
			props.size.0, props.size.1, &props.title, glfw::WindowMode::Windowed
		).unwrap();

		// TODO: OpenGL specific code should not be here
		gl::load_with(|s| window.get_proc_address(s));

		window.set_close_polling(true);
		window.set_size_polling(true);
		window.set_key_polling(true);
		window.set_char_polling(true);
		window.set_mouse_button_polling(true);
		window.set_cursor_pos_polling(true);
		window.set_scroll_polling(true);

		Self {
			inner: window,
			evt_recv,
			title: props.title,
			close_requested: false,
		}
	}
}

impl Window for GLFWWindow {
	fn set_title(&mut self, title: String) {
		self.inner.set_title(&title);
		self.title = title;
	}
	fn get_title(&self) -> &str {
		&self.title
	}
	fn get_size(&self) -> (u32, u32) {
		let (w, h) = self.inner.get_size();
		(w as _, h as _)
	}
	fn swap_buffers(&mut self) {
		use glfw::Context;
		self.inner.swap_buffers();
	}
	fn update(&mut self) -> Vec<Box<dyn Event>> {
		let mut events: Vec<Box<dyn Event>> = vec![];

		glfw_handle().poll_events();

		for(_, event) in glfw::flush_messages(&self.evt_recv) {
			match event {
				glfw::WindowEvent::Close => {
					events.push(WindowCloseRequestedEvent::boxed());
					self.close_requested = true;
				}
				glfw::WindowEvent::Size(w, h) => {
					events.push(WindowResizedEvent::boxed((w as _, h as _)));
				}
				glfw::WindowEvent::Key(key, _scancode, glfw::Action::Press, _modifiers) => {
					events.push(KeyPressedEvent::boxed(convert_key_event(key), false));
				}
				glfw::WindowEvent::Key(key, _scancode, glfw::Action::Repeat, _modifiers) => {
					events.push(KeyPressedEvent::boxed(convert_key_event(key), true));
				}
				glfw::WindowEvent::Key(key, _scancode, glfw::Action::Release, _modifiers) => {
					events.push(KeyReleasedEvent::boxed(convert_key_event(key)));
				}
				glfw::WindowEvent::Char(c) => {
					events.push(TextWrittenEvent::boxed(c));
				}
				glfw::WindowEvent::MouseButton(button, glfw::Action::Press, _modifiers) => {
					events.push(MouseDownEvent::boxed(match button {
						glfw::MouseButton::Button1 => Button::MouseLeft,
						glfw::MouseButton::Button2 => Button::MouseRight,
						glfw::MouseButton::Button3 => Button::MouseMiddle,
						glfw::MouseButton::Button4 => Button::Mouse4,
						glfw::MouseButton::Button5 => Button::Mouse5,
						glfw::MouseButton::Button6 => Button::Mouse6,
						glfw::MouseButton::Button7 => Button::Mouse7,
						glfw::MouseButton::Button8 => Button::Mouse8,
					}));
				}
				glfw::WindowEvent::MouseButton(button, glfw::Action::Release, _modifiers) => {
					events.push(MouseUpEvent::boxed(match button {
						glfw::MouseButton::Button1 => Button::MouseLeft,
						glfw::MouseButton::Button2 => Button::MouseRight,
						glfw::MouseButton::Button3 => Button::MouseMiddle,
						glfw::MouseButton::Button4 => Button::Mouse4,
						glfw::MouseButton::Button5 => Button::Mouse5,
						glfw::MouseButton::Button6 => Button::Mouse6,
						glfw::MouseButton::Button7 => Button::Mouse7,
						glfw::MouseButton::Button8 => Button::Mouse8,
					}));
				}
				glfw::WindowEvent::CursorPos(x, y) => {
					let pos: (u32, u32) = (x as u32, y as u32);
					let old_pos = get_mouse_position();
					let delta: (i32, i32) = (
						pos.0 as i32 - old_pos.0 as i32,
						pos.1 as i32 - old_pos.1 as i32,
					);
					events.push(MouseMoveEvent::boxed(pos, delta));
				}
				glfw::WindowEvent::Scroll(x, y) => {
					events.push(MouseScrolledEvent::boxed((x as Float, y as Float)));
				}
				_ => {}
			}
		}

		events
	}
	fn close_requested(&self) -> bool {
		self.close_requested
	}
}

fn convert_key_event(key: glfw::Key) -> Button {
	match key {
		glfw::Key::Space => Button::Space,
		glfw::Key::Apostrophe => Button::Apostrophe,
		glfw::Key::Comma => Button::Comma,
		glfw::Key::Minus => Button::Minus,
		glfw::Key::Period => Button::Period,
		glfw::Key::Slash => Button::Slash,
		glfw::Key::Num0 => Button::Num0,
		glfw::Key::Num1 => Button::Num1,
		glfw::Key::Num2 => Button::Num2,
		glfw::Key::Num3 => Button::Num3,
		glfw::Key::Num4 => Button::Num4,
		glfw::Key::Num5 => Button::Num5,
		glfw::Key::Num6 => Button::Num6,
		glfw::Key::Num7 => Button::Num7,
		glfw::Key::Num8 => Button::Num8,
		glfw::Key::Num9 => Button::Num9,
		glfw::Key::Semicolon => Button::Semicolon,
		glfw::Key::Equal => Button::Equals,
		glfw::Key::A => Button::A,
		glfw::Key::B => Button::B,
		glfw::Key::C => Button::C,
		glfw::Key::D => Button::D,
		glfw::Key::E => Button::E,
		glfw::Key::F => Button::F,
		glfw::Key::G => Button::G,
		glfw::Key::H => Button::H,
		glfw::Key::I => Button::I,
		glfw::Key::J => Button::J,
		glfw::Key::K => Button::K,
		glfw::Key::L => Button::L,
		glfw::Key::M => Button::M,
		glfw::Key::N => Button::N,
		glfw::Key::O => Button::O,
		glfw::Key::P => Button::P,
		glfw::Key::Q => Button::Q,
		glfw::Key::R => Button::R,
		glfw::Key::S => Button::S,
		glfw::Key::T => Button::T,
		glfw::Key::U => Button::U,
		glfw::Key::V => Button::V,
		glfw::Key::W => Button::W,
		glfw::Key::X => Button::X,
		glfw::Key::Y => Button::Y,
		glfw::Key::Z => Button::Z,
		glfw::Key::LeftBracket => Button::BracketLeft,
		glfw::Key::Backslash => Button::Backslash,
		glfw::Key::RightBracket => Button::BracketRight,
		glfw::Key::GraveAccent => Button::Tilde,
		glfw::Key::Escape => Button::Escape,
		glfw::Key::Enter => Button::Enter,
		glfw::Key::Tab => Button::Tab,
		glfw::Key::Backspace => Button::Backspace,
		glfw::Key::Insert => Button::Insert,
		glfw::Key::Delete => Button::Delete,
		glfw::Key::Right => Button::ArrowRight,
		glfw::Key::Left => Button::ArrowLeft,
		glfw::Key::Down => Button::ArrowDown,
		glfw::Key::Up => Button::ArrowUp,
		glfw::Key::PageUp => Button::PgUp,
		glfw::Key::PageDown => Button::PgDown,
		glfw::Key::Home => Button::Home,
		glfw::Key::End => Button::End,
		glfw::Key::CapsLock => Button::CapsLock,
		glfw::Key::ScrollLock => Button::ScrollLock,
		glfw::Key::NumLock => Button::NumLock,
		glfw::Key::PrintScreen => Button::PrintScreen,
		glfw::Key::Pause => Button::Pause,
		glfw::Key::F1 => Button::F1,
		glfw::Key::F2 => Button::F2,
		glfw::Key::F3 => Button::F3,
		glfw::Key::F4 => Button::F4,
		glfw::Key::F5 => Button::F5,
		glfw::Key::F6 => Button::F6,
		glfw::Key::F7 => Button::F7,
		glfw::Key::F8 => Button::F8,
		glfw::Key::F9 => Button::F9,
		glfw::Key::F10 => Button::F10,
		glfw::Key::F11 => Button::F11,
		glfw::Key::F12 => Button::F12,
		glfw::Key::Kp0 => Button::NumPad0,
		glfw::Key::Kp1 => Button::NumPad1,
		glfw::Key::Kp2 => Button::NumPad2,
		glfw::Key::Kp3 => Button::NumPad3,
		glfw::Key::Kp4 => Button::NumPad4,
		glfw::Key::Kp5 => Button::NumPad5,
		glfw::Key::Kp6 => Button::NumPad6,
		glfw::Key::Kp7 => Button::NumPad7,
		glfw::Key::Kp8 => Button::NumPad8,
		glfw::Key::Kp9 => Button::NumPad9,
		glfw::Key::KpDecimal => Button::NumPadDec,
		glfw::Key::KpDivide => Button::NumPadDiv,
		glfw::Key::KpMultiply => Button::NumPadMult,
		glfw::Key::KpSubtract => Button::NumPadSub,
		glfw::Key::KpAdd => Button::NumPadAdd,
		glfw::Key::KpEnter => Button::NumPadEnter,
		glfw::Key::KpEqual => Button::NumPadEq,
		glfw::Key::LeftShift => Button::LShift,
		glfw::Key::LeftControl => Button::LCtrl,
		glfw::Key::LeftAlt => Button::LAlt,
		glfw::Key::LeftSuper => Button::LSuper,
		glfw::Key::RightShift => Button::RShift,
		glfw::Key::RightControl => Button::RCtrl,
		glfw::Key::RightAlt => Button::RAlt,
		glfw::Key::RightSuper => Button::RSuper,
		glfw::Key::Menu => Button::Menu,
		glfw::Key::Unknown => Button::Unknown,

		glfw::Key::World1 => Button::Unknown,
		glfw::Key::World2 => Button::Unknown,
		glfw::Key::F13 => Button::Unknown,
		glfw::Key::F14 => Button::Unknown,
		glfw::Key::F15 => Button::Unknown,
		glfw::Key::F16 => Button::Unknown,
		glfw::Key::F17 => Button::Unknown,
		glfw::Key::F18 => Button::Unknown,
		glfw::Key::F19 => Button::Unknown,
		glfw::Key::F20 => Button::Unknown,
		glfw::Key::F21 => Button::Unknown,
		glfw::Key::F22 => Button::Unknown,
		glfw::Key::F23 => Button::Unknown,
		glfw::Key::F24 => Button::Unknown,
		glfw::Key::F25 => Button::Unknown,
	}
}
