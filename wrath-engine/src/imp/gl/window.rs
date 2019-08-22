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
	fn update(&mut self) {
		// Temporary work-around, should use
		// some event queue in the future.
		let mut close_requested = false;

		self.evt_loop.poll_events(|event| match event {
			glutin::Event::WindowEvent { event, .. } => match event {
				glutin::WindowEvent::CloseRequested => {
					close_requested = true;
				}
				_ => {}
			},
			_ => {}
		});

		if close_requested {
			self.close_requested = true;
		}
	}
	fn close_requested(&self) -> bool {
		self.close_requested
	}
}
