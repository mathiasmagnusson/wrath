use crate::Event;

pub trait Window {
	fn set_title(&mut self, title: String);
	fn get_title(&self) -> &str;
	fn get_size(&self) -> (u32, u32);
	fn update(&mut self) -> Vec<Box<dyn Event>>;
	fn close_requested(&self) -> bool;
}

pub struct WindowProps {
	pub title: String,
	pub size: (u32, u32),
}

pub fn create(props: WindowProps) -> Box<dyn Window> {
	box crate::imp::gl::Window::new(props)
}
