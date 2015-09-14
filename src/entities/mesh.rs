use super::vector3d::Vector3d;
use super::geometry::Geometry;

pub struct Mesh {
	pub position: Vector3d,
	pub rotation: Vector3d,
	pub geometry: Geometry
}

impl Mesh {
	pub fn new(geo: Geometry) -> Mesh {
		Mesh{
			position: Vector3d::new(0.0, 0.0, 0.0),
			rotation: Vector3d::new(0.0, 0.0, 0.0),
			geometry: geo
		}
	}

	// pub fn set_rotation_x(&mut self, angle: f64) {
	// 	self.rotation.set_x(angle);
	// }

	// pub fn set_rotation_y(&mut self, angle: f64) {
	// 	self.rotation.set_y(angle);
	// }

	// pub fn set_rotation_z(&mut self, angle: f64) {
	// 	self.rotation.set_z(angle);
	// }
}

#[cfg(test)]
mod tests {
	use super::*;
	// use super::super::paddle::{Paddle};

	// #[test]
}