use std::ffi::CString;

use crate::events::*;
use crate::input::get_mouse_position;
use crate::Button;
use crate::Event;
use crate::Float;
use crate::WindowProps;

pub struct X11Window {
	display: *mut x11::xlib::Display,
	window: u64,
	title: String,
	close_requested: bool,
}

impl X11Window {
	pub fn new(props: WindowProps) -> Self {
		unsafe {
			use x11::xlib::*;

			let display = XOpenDisplay(std::ptr::null());
			assert!(display != std::ptr::null_mut(), "XOpenDisplay returned null");

			let screen = XDefaultScreenOfDisplay(display);
			assert!(display != std::ptr::null_mut(), "XDefaultScreenOfDisplay returned null");

			let screen_id = XDefaultScreen(display);
			// assert!();

			let white = XWhitePixel(display, screen_id);
			let black = XBlackPixel(display, screen_id);
			let root = XRootWindowOfScreen(screen);
			let x = 0;
			let y = 0;
			let window = XCreateSimpleWindow(display, root, x, y, props.size.0, props.size.1, 1, white, black);

			let title = CString::new(props.title.clone()).expect("Window title had a null byte in it");

			XStoreName(display, window, title.as_ptr());
			XSetIconName(display, window, title.as_ptr());

			XClearWindow(display, window);
			XMapRaised(display, window);
		
			Self {
				display,
				window,
				title: props.title,
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
	}
	fn get_title(&self) -> &str {
		&self.title
	}
	fn get_size(&self) -> (u32, u32) {
		unimplemented!();
	}
	fn close_requested(&self) -> bool {
		self.close_requested
	}
	fn update(&mut self) -> Vec<Box<dyn Event>> {
		let mut events: Vec<Box<dyn Event>> = vec![];

		// https://github.com/gamedevtech/X11OpenGLWindow/blob/master/README.md

		events
	}
}
