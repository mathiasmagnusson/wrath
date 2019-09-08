use crate::Float;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec4 {
	v: [Float; 4]
}

impl Vec4 {
	pub const fn new(v0: Float, v1: Float, v2: Float, v3: Float) -> Self {
		Self { v: [v0, v1, v2, v3] }
	}
	pub fn x(&self) -> Float {
		self[0]
	}
	pub fn y(&self) -> Float {
		self[1]
	}
	pub fn z(&self) -> Float {
		self[2]
	}
	pub fn w(&self) -> Float {
		self[3]
	}
	pub fn r(&self) -> Float {
		self[0]
	}
	pub fn g(&self) -> Float {
		self[1]
	}
	pub fn b(&self) -> Float {
		self[2]
	}
	pub fn a(&self) -> Float {
		self[3]
	}
	pub fn len() -> usize {
		4
	}
	pub fn mag_sq(&self) -> Float {
		self[0] * self[0] +
		self[1] * self[1] +
		self[2] * self[2] +
		self[3] * self[3]
	}
	pub fn mag(&self) -> Float {
		self.mag_sq().sqrt()
	}
	pub fn normalized(&self) -> Vec4 {
		let mag = self.mag();
		Vec4 {
			v: [
				self[0] / mag,
				self[1] / mag,
				self[2] / mag,
				self[3] / mag,
			]
		}
	}
	pub fn normalize(&mut self) {
		let mag = self.mag();
		self[0] /= mag;
		self[1] /= mag;
		self[2] /= mag;
	}
}

impl From<(Float, Float, Float, Float)> for Vec4 {
	fn from(v: (Float, Float, Float, Float)) -> Self {
		Self {
			v: [
				v.0,
				v.1,
				v.2,
				v.3,
			]
		}
	}
}

impl std::ops::Index<usize> for Vec4 {
	type Output = Float;
	fn index(&self, i: usize) -> &Self::Output {
		&self.v[i]
	}
}

impl std::ops::IndexMut<usize> for Vec4 {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.v[i]
	}
}

impl std::ops::Add for Vec4 {
	type Output = Self;
	fn add(self, other: Self) -> Self::Output {
		Self {
			v: [
				self[0] + other[0],
				self[1] + other[1],
				self[2] + other[2],
				self[3] + other[3],
			]
		}
	}
}

impl std::ops::Sub for Vec4 {
	type Output = Self;
	fn sub(self, other: Self) -> Self::Output {
		Self {
			v: [
				self[0] - other[0],
				self[1] - other[1],
				self[2] - other[2],
				self[3] - other[3],
			]
		}
	}
}

impl std::ops::Mul<Float> for Vec4 {
	type Output = Self;
	fn mul(self, scalar: Float) -> Self::Output {
		Self {
			v: [
				self[0] * scalar,
				self[1] * scalar,
				self[2] * scalar,
				self[3] * scalar,
			]
		}
	}
}

impl std::ops::Div<Float> for Vec4 {
	type Output = Self;
	fn div(self, scalar: Float) -> Self::Output {
		Self {
			v: [
				self[0] / scalar,
				self[1] / scalar,
				self[2] / scalar,
				self[3] / scalar,
			]
		}
	}
}

impl std::ops::AddAssign for Vec4 {
	fn add_assign(&mut self, other: Self) {
		self[0] += other[0];
		self[1] += other[1];
		self[2] += other[2];
		self[3] += other[3];
	}
}

impl std::ops::SubAssign for Vec4 {
	fn sub_assign(&mut self, other: Self) {
		self[0] -= other[0];
		self[1] -= other[1];
		self[2] -= other[2];
		self[3] -= other[3];
	}
}

impl std::ops::MulAssign<Float> for Vec4 {
	fn mul_assign(&mut self, scalar: Float) {
		self[0] *= scalar;
		self[1] *= scalar;
		self[2] *= scalar;
		self[3] *= scalar;
	}
}

impl std::ops::DivAssign<Float> for Vec4 {
	fn div_assign(&mut self, scalar: Float) {
		self[0] /= scalar;
		self[1] /= scalar;
		self[2] /= scalar;
		self[3] /= scalar;
	}
}
