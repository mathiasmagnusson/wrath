#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OverlayHandle(u32);

impl OverlayHandle {
	pub fn none() -> Self {
		Self(0)
	}
	pub fn new(value: u32) -> Self {
		Self(value)
	}
}
