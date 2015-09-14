extern crate sdl2;

use super::vector3d::Vector3d;
use super::vector2d::Vector2d;
// use std::f64::{consts};

// use self::sdl2::rect::Point;

pub struct Camera {
	pub fl: f64,
	pub fov: f64,
	pub aspect: f64,
	pub position: Vector3d,
	pub rotation: Vector3d
}

impl Camera {
	pub fn new(position: Vector3d, fov: f64, fl: f64, aspect: f64) -> Camera {
		Camera{
			fov: fov,
			fl: fl,
			aspect: aspect,
			position: position,
			rotation: Vector3d::new(0.0, 0.0, 0.0)
		}
	}

	pub fn set_aspect(&mut self, aspect: f64) {
		self.aspect = aspect;
	}

	// pub fn get_relative_coordinates(&mut self, vec: &Vector3d) -> Vector3d {
	// 	let diff = self.position.difference(&vec);
	// 	diff
	// }

	pub fn project_to_2d(&mut self, vec: Vector3d, width: f64, height: f64) -> (Vector2d, bool) {

		let rel = Vector3d::new(
			vec.x - self.position.x, 
			vec.y - self.position.y, 
			vec.z - self.position.z
		);

		let mut trans_y: f64;
		let mut trans_x: f64;

		let fov_h = self.fov * 0.5;
		let fov_w = fov_h * self.aspect;
		// let view_width = self.fl * fov_w.tan();
		// let view_height = self.fl * fov_h.tan();

		let mut out_of_bounds = false;

		trans_x = (rel.x / (rel.z + self.fl)).atan() / (fov_w);

		if trans_x > 1.3 {
			out_of_bounds = true;
		}

		if rel.x < 0.0 {
			- trans_x;
		}

		trans_y = (rel.y / (rel.z + self.fl)).atan() / (fov_h);

		if trans_y > 1.3 {
			out_of_bounds = true;
		}

		if rel.y < 0.0 {
			- trans_y;
		}

		trans_x *= 0.5;
		trans_y *= 0.5;

		trans_x += 0.5;
		trans_y += 0.5;

		if vec.z < 0.0 {
			out_of_bounds = true;
		}

		// fl * tan( theta ) = height;

		return (Vector2d::new((trans_x * width), (trans_y * height)), out_of_bounds);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	// use super::super::paddle::{Paddle};

	// #[test]
}