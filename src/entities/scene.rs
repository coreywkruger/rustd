use super::mesh::Mesh;

pub struct Scene {
	pub objects: Vec<Mesh>
}

impl Scene {
	pub fn new() -> Scene {
		Scene{
			objects: Vec::new() 
		}
	}

	pub fn add_object(&mut self, mesh: Mesh) {
		self.objects.push(mesh);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	// use super::super::paddle::{Paddle};

	// #[test]
}