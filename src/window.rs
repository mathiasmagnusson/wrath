pub trait Window {
	fn set_title(&mut self, title: String);
	fn get_title(&self) -> &str;
	fn get_size(&self) -> (u32, u32);
	fn update(&mut self);
}

pub struct WindowProps {
	pub title: String,
	pub size: (u32, u32),
}
