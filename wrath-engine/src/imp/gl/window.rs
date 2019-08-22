pub struct Window {
	inner: glutin::Window,
	evt_loop: glutin::EventsLoop,
	title: String,
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
		self.evt_loop.poll_events(|event| {
			match event {
				glutin::Event::WindowEvent { event, .. } => {
					match event {
						glutin::WindowEvent::CloseRequested => {
							println!("time to do some real shit");
						}
						_ => {}
					}
				},
				_ => {},
			}
		});
	}
}
