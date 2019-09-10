use crate::{events::*, input::get_mouse_position, Button, Event, Float, WindowProps};

use std::ffi::CString;

pub struct X11Window {
	display: *mut x11::xlib::Display,
	window: u64,
	title: String,
	size: (u32, u32),
	close_requested: bool,
}

impl X11Window {
	pub fn new(props: WindowProps) -> Self {
		unsafe {
			use x11::xlib::*;

			let display = XOpenDisplay(std::ptr::null());
			assert!(
				display != std::ptr::null_mut(),
				"XOpenDisplay returned null"
			);

			let screen = XDefaultScreenOfDisplay(display);
			assert!(
				display != std::ptr::null_mut(),
				"XDefaultScreenOfDisplay returned null"
			);

			let screen_id = XDefaultScreen(display);
			// assert!();

			let white = XWhitePixel(display, screen_id);
			let black = XBlackPixel(display, screen_id);
			let root = XRootWindowOfScreen(screen);
			let x = 0;
			let y = 0;
			let window = XCreateSimpleWindow(
				display,
				root,
				x,
				y,
				props.size.0,
				props.size.1,
				1,
				white,
				black,
			);

			let title =
				CString::new(props.title.clone()).expect("Window title had a null byte in it");

			XStoreName(display, window, title.as_ptr());
			XSetIconName(display, window, title.as_ptr());

			XSelectInput(
				display,
				window,
				KeyPressMask | KeyReleaseMask | KeymapStateMask,
			);

			XClearWindow(display, window);
			XMapRaised(display, window);
			Self {
				display,
				window,
				title: props.title,
				size: props.size,
				close_requested: false,
			}
		}
	}
}

impl Drop for X11Window {
	fn drop(&mut self) {
		unsafe {
			use x11::xlib::*;

			XDestroyWindow(self.display, self.window);
			XCloseDisplay(self.display);
		}
	}
}

impl crate::Window for X11Window {
	fn set_title(&mut self, title: String) {
		self.title = title;
		unsafe {
			use x11::xlib::*;
			let title =
				CString::new(self.title.clone()).expect("Window title had a null byte in it");
			XStoreName(self.display, self.window, title.as_ptr());
			XSetIconName(self.display, self.window, title.as_ptr());
		}
	}
	fn get_title(&self) -> &str {
		&self.title
	}
	fn get_size(&self) -> (u32, u32) {
		self.size
	}
	fn close_requested(&self) -> bool {
		self.close_requested
	}
	fn update(&mut self) -> Vec<Box<dyn Event>> {
		let mut events: Vec<Box<dyn Event>> = vec![];

		unsafe {
			use x11::xlib;

			let mut event = xlib::XEvent { type_: 0 };

			xlib::XNextEvent(self.display, &mut event as *mut xlib::XEvent);

			match event.type_ {
				xlib::KeymapNotify => {
					xlib::XRefreshKeyboardMapping(&mut event.mapping as *mut xlib::XMappingEvent);
				}
				xlib::KeyPress => {
					let mut typed = [0u8; 25];
					let mut keysym: xlib::KeySym = 0;
					let len = xlib::XLookupString(
						&mut event.key as *mut xlib::XKeyEvent,
						typed.as_mut_ptr() as *mut i8,
						typed.len() as _,
						&mut keysym as *mut xlib::KeySym,
						std::ptr::null_mut(),
					);

					let button = convert_key_event(keysym);
					events.push(KeyPressedEvent::boxed(button, button.is_pressed()));
				}
				_ => {}
			}
		}

		events
	}
}

fn convert_key_event(keysym: x11::xlib::KeySym) -> Button {
	use x11::keysym;
	match keysym as u32 {
		keysym::XK_a | keysym::XK_A => Button::A,
		keysym::XK_b | keysym::XK_B => Button::B,
		keysym::XK_c | keysym::XK_C => Button::C,
		keysym::XK_d | keysym::XK_D => Button::D,
		keysym::XK_e | keysym::XK_E => Button::E,
		keysym::XK_f | keysym::XK_F => Button::F,
		keysym::XK_g | keysym::XK_G => Button::G,
		keysym::XK_h | keysym::XK_H => Button::H,
		keysym::XK_i | keysym::XK_I => Button::I,
		keysym::XK_j | keysym::XK_J => Button::J,
		keysym::XK_k | keysym::XK_K => Button::K,
		keysym::XK_l | keysym::XK_L => Button::L,
		keysym::XK_m | keysym::XK_M => Button::M,
		keysym::XK_n | keysym::XK_N => Button::N,
		keysym::XK_o | keysym::XK_O => Button::O,
		keysym::XK_p | keysym::XK_P => Button::P,
		keysym::XK_q | keysym::XK_Q => Button::Q,
		keysym::XK_r | keysym::XK_R => Button::R,
		keysym::XK_s | keysym::XK_S => Button::S,
		keysym::XK_t | keysym::XK_T => Button::T,
		keysym::XK_u | keysym::XK_U => Button::U,
		keysym::XK_v | keysym::XK_V => Button::V,
		keysym::XK_w | keysym::XK_W => Button::W,
		keysym::XK_x | keysym::XK_X => Button::X,
		keysym::XK_y | keysym::XK_Y => Button::Y,
		keysym::XK_z | keysym::XK_Z => Button::Z,
		_ => Button::Unknown,
	}
}
