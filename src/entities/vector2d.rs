pub struct Vector2d {
	pub x: f64,
	pub y: f64
}

impl Vector2d {
	pub fn new(x: f64, y: f64) -> Vector2d {
		Vector2d{
			x: x,
			y: y
		}
	}

	pub fn sign(vec1: &Vector2d, vec2: &Vector2d, vec3: &Vector2d) -> f64 {
	    return 
	    	(vec1.x - vec3.x) * (vec2.y - vec3.y) - 
	    	(vec2.x - vec3.x) * (vec1.y - vec3.y);
	}

	pub fn get_length(&mut self) -> f64 {
		return (self.x * self.x + self.y * self.y).abs().sqrt();
	}

	pub fn normalize(&mut self) -> Vector2d {
		let length = self.get_length();
		return Vector2d::new(self.x.abs() / length, self.y.abs() / length);
	}

	pub fn distance_to(&mut self, vec: Vector2d) -> f64 {
		return self.difference(vec).get_length();
	}

	pub fn difference(&mut self, vec: Vector2d) -> Vector2d {
		return Vector2d::new(vec.x - self.x, vec.y - self.y);
	}

	pub fn clone(&mut self) -> Vector2d {
		return Vector2d::new(self.x, self.y);
	}

	pub fn add(&mut self, vec: Vector2d) {
		self.x += vec.x;
		self.y += vec.y;
	}

	pub fn set(&mut self, x: f64, y: f64) {
		self.x = x;
		self.y = y;
	}

	pub fn set_x(&mut self, x: f64) {
		self.x = x;
	}

	pub fn set_y(&mut self, y: f64) {
		self.y = y;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	// use super::super::paddle::{Paddle};

	// #[test]
}