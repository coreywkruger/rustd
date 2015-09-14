use std::f64::{consts};

// #[derive(Debug, Copy, Clone)]
pub struct Vector3d {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

// impl Copy for Vector3d {}
// impl Clone for Vector3d { 
// 	fn clone(&self) -> Vector3d { *self } 
// 	}

// impl Clone for Vector3d {}
impl Vector3d {
	pub fn new(x: f64, y: f64, z: f64) -> Vector3d {
		Vector3d{
			x: x,
			y: y,
			z: z
		}
	}


	pub fn get_length(&mut self) -> f64 {
		return (self.x * self.x + self.y * self.y + self.z * self.z).abs().sqrt();
	}

	pub fn normalize(&mut self) -> Vector3d {
		let length = self.get_length();
		return Vector3d::new(self.x / length, self.y / length, self.z / length);
	}

	pub fn distance_to(&mut self, vec: Vector3d) -> f64 {
		return self.difference(&vec).get_length();
	}

	pub fn difference(&mut self, vec: &Vector3d) -> Vector3d {
		return Vector3d::new(vec.x - self.x, vec.y - self.y, vec.z - self.z);
	}

	// pub fn add(&mut self, vec: Vector3d) {
	// 	self.x += vec.x;
	// 	self.y += vec.y;
	// 	self.z += vec.z;
	// }
	// pub fn add(&mut self, x: f64, y: f64, z: f64) {
	pub fn add(&mut self, vec: &Vector3d) {
		self.x += vec.x;
		self.y += vec.y;
		self.z += vec.z;
	}

	pub fn multiply(&mut self, vec: &Vector3d) {
		self.x *= vec.x;
		self.y *= vec.y;
		self.z *= vec.z;
	}

	pub fn sub(&mut self, vec: Vector3d) {
		self.x -= vec.x;
		self.y -= vec.y;
		self.z -= vec.z;
	}

	pub fn multiply_scalar(vec: Vector3d, scalar: f64) -> Vector3d {
		return Vector3d::new(vec.x * scalar, vec.y * scalar, vec.z * scalar); 
	}

	pub fn set_x(&mut self, x: f64) {
		self.x = x;
	}

	pub fn set_y(&mut self, y: f64) {
		self.y = y;
	}

	pub fn set_z(&mut self, z: f64) {
		self.z = z;
	}

	pub fn set(&mut self, x: f64, y: f64, z: f64) {
		self.x = x;
		self.y = y;
		self.z = z;
	}

	pub fn get_x_axis(euler: &Vector3d) -> Vector3d {
		let mut rotated = Vector3d::new(1.0, 0.0, 0.0);
		rotated.rotate_x(euler.x);
		rotated
	}

	pub fn get_y_axis(euler: &Vector3d) -> Vector3d {
		let mut rotated = Vector3d::new(0.0, 1.0, 0.0);
		rotated.rotate_y(euler.y);
		rotated
	}

	pub fn get_z_axis(euler: &Vector3d) -> Vector3d {
		let mut rotated = Vector3d::new(0.0, 0.0, 1.0);
		rotated.rotate_z(euler.z);
		rotated
	}

	pub fn dot(&mut self, vec: &Vector3d) -> f64 {
		return self.x * vec.x + self.y * vec.y + self.z * vec.z; 
	}

	pub fn cross(&mut self, vec:  &Vector3d) -> Vector3d {
		return Vector3d::new( 
			self.y * vec.z - self.z * vec.y,
			self.z * vec.x - self.x * vec.z,
			self.x * vec.y - self.y * vec.x
		);
	}

	pub fn set_length(&mut self, length: f64) {
		
		let normed = self.normalize();

		self.x = normed.x * length;
		self.y = normed.y * length;
		self.z = normed.z * length;
	}

	pub fn copy(&mut self, vec: &Vector3d) {
		self.x = vec.x;
		self.y = vec.y;
		self.z = vec.z;
	}

	pub fn rotate_on_axis(&mut self, axis: &Vector3d, angle: f64) {

		let mut projection = Vector3d::new(axis.x, axis.y, axis.z);
		let dot = self.dot(&axis);
		projection.set_length(dot);

		let mut diff = Vector3d::new(self.x - projection.x, self.y - projection.y, self.z - projection.z);
		let mut green = self.cross(&axis).normalize();
		let cross = green.cross(&axis);
		self.copy(&cross);

		green.set_length(diff.get_length() * angle.sin());
		self.set_length(diff.get_length() * angle.cos());
		self.add(&green);
		self.add(&projection);
	}

	pub fn rotate_x(&mut self, angle: f64) {
		let hyp = (self.z * self.z + self.y * self.y).sqrt();
		let mut phi: f64 = 0.0;
		if hyp != 0.0 {
			phi = (self.y.abs() / hyp).asin();  // quadrant 1
		}
		if self.z < 0.0 && self.y > 0.0 { // quadrant 2
			phi += (1.0 * consts::PI) / 2.0;
		} else 
		if self.z < 0.0 && self.y < 0.0 { // quadrant 3
			phi += (2.0 * consts::PI) / 2.0;
		} else 
		if self.z > 0.0 && self.y < 0.0 { // quadrant 4
			phi += (3.0 * consts::PI) / 2.0;
		}
		let new_z = (phi + angle).cos() * hyp;
		let new_y = (phi + angle).sin() * hyp;

		self.y = new_y;
		self.z = new_z;
	}

	pub fn rotate_y(&mut self, angle: f64) {
		let hyp = (self.x * self.x + self.z * self.z).sqrt();
		let mut phi: f64 = 0.0;
		if hyp != 0.0 {
			phi = (self.z / hyp).asin(); // quadrant 1
		}
		if self.x < 0.0 && self.z > 0.0 { // quadrant 2
			phi += 1.0 * consts::PI / 2.0;
		} else if self.x < 0.0 && self.z < 0.0 { // quadrant 3
			phi += 2.0 * consts::PI / 2.0;
		} else if self.x > 0.0 && self.z < 0.0 { // quadrant 4
			phi += 3.0 * consts::PI / 2.0;
		}

		// let resized: f64 = angle;
		// if angle > consts::PI * 2.0 {
		// 	resized = angle - consts::PI * 2.0;
		// } 
		// println!("||||| {:?}", resized);
		let new_x = (phi + angle).cos() * hyp;
		let new_z = (phi + angle).sin() * hyp;

		self.x = new_x;
		self.z = new_z;
	}

	pub fn rotate_z(&mut self, angle: f64) {
		let hyp = (self.x * self.x + self.y * self.y).sqrt();
		let mut phi: f64 = 0.0;
		if hyp != 0.0 {
			phi = (self.y.abs() / hyp).asin(); // quadrant 1
		}
		if self.x < 0.0 && self.y > 0.0 { // quadrant 2
			phi += 1.0 * consts::PI / 2.0;
		} else if self.x < 0.0 && self.y < 0.0 { // quadrant 3
			phi += 2.0 * consts::PI / 2.0;
		} else if self.x > 0.0 && self.y < 0.0 { // quadrant 4
			phi += 3.0 * consts::PI / 2.0;
		}
		let new_x = (phi + angle).cos() * hyp;
		let new_y = (phi + angle).sin() * hyp;

		self.x = new_x;
		self.y = new_y;
	}
}

// impl Copy for Vector3d {}
// impl Clone for Vector3d {}

#[cfg(test)]
mod tests {
	use super::*;
	// use super::super::paddle::{Paddle};

	// #[test]
}