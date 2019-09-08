#[cfg(feature = "big-floats")]
pub type Float = f64;
#[cfg(not(feature = "big-floats"))]
pub type Float = f32;

mod vec3;
mod vec4;

pub use vec3::Vec3;
pub use vec4::Vec4;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
