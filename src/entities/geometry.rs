use super::vector3d::Vector3d;
use super::face::Face;

pub struct Geometry{
	pub vertices: Vec<Vector3d>,
	// pub edges: Vec<[usize; 2]>,
	pub faces: Vec<Face>
}

impl Geometry {
	pub fn new(verts: Vec<Vector3d>, /*edges: Vec<[usize; 2]>,*/ faces: Vec<Face>) -> Geometry {

		Geometry{
			vertices: verts,
			// edges: edges,
			faces: faces
		}
	}

	pub fn new_cube_geometry() -> Geometry {
		let mut vecs = Vec::new();

		vecs.push(Vector3d::new(5.0, 5.0, 5.0)); // 0
		vecs.push(Vector3d::new(-5.0, 5.0, 5.0)); // 1
		vecs.push(Vector3d::new(-5.0, 5.0, -5.0)); // 2
		vecs.push(Vector3d::new(5.0, 5.0, -5.0)); // 3

		vecs.push(Vector3d::new(5.0, -5.0, 5.0)); // 4
		vecs.push(Vector3d::new(-5.0, -5.0, 5.0)); // 5
		vecs.push(Vector3d::new(-5.0, -5.0, -5.0)); // 6
		vecs.push(Vector3d::new(5.0, -5.0, -5.0)); // 7

		// let mut edges = vec![];

		// edges.push([0, 1]);
		// edges.push([1, 2]);
		// edges.push([2, 3]);
		// edges.push([3, 0]);

		// edges.push([0, 4]);
		// edges.push([1, 5]);
		// edges.push([2, 6]);
		// edges.push([3, 7]);

		// edges.push([4, 5]);
		// edges.push([5, 6]);
		// edges.push([6, 7]);
		// edges.push([7, 4]);

		let mut faces = vec![];

		// Face 1
		faces.push(Face::new(0, 1, 2));
		faces.push(Face::new(0, 2, 3));

		// Face 2
		faces.push(Face::new(0, 1, 4));
		faces.push(Face::new(1, 5, 4));

		// Face 3
		faces.push(Face::new(1, 2, 5));
		faces.push(Face::new(2, 6, 5));

		//Face 4
		faces.push(Face::new(2, 3, 6));
		faces.push(Face::new(3, 7, 6));

		//Face 5
		faces.push(Face::new(0, 3, 4));
		faces.push(Face::new(3, 7, 4));

		// Face 6
		faces.push(Face::new(4, 5, 6));
		faces.push(Face::new(4, 6, 7));

		return Geometry::new(vecs, faces);
	}

	pub fn add_vertex(&mut self, vertex: Vector3d) {
		self.vertices.push(vertex);
	}

	pub fn add_face(&mut self, face: Face) {
		self.faces.push(face);
	}

	// pub fn new_plane_geometry(segments: i32, width: f64) -> Geometry {
	// 	let mut vecs = Vec::new();

	// 	vecs.push(Vector3d::new(width, width, 0.0)); // 0

	// 	let mut edges = vec![];

	// 	for seg1 in 0..segments {
	// 		for seg2 in 0..segments {

				
	// 			// if(vecs.len() < 100) {
	// 			// 	// println!("INDICES {}", (seg1 * segments + seg2));
	// 			// 	edges.push([(seg1 * segments + seg2) as i32, (seg1 * segments + seg2 + 1) as i32]);
	// 			// 	// println!("INDICES------- {}", "blah");
	// 			// }

	// 			// println!("INDICES------- {}", (- 0.5 * width));

	// 			let mut seg_widh = width / (segments as f64);
	// 			let mut new_vec = Vector3d::new(
	// 				(- 0.5 * width) + seg_widh * (seg1 as f64),
	// 				(- 0.5 * width) + seg_widh * (seg2 as f64),
	// 				0.0
	// 			);

	// 			vecs.push(new_vec);
	// 			// println!("AAAA {} {} {} ", new_vec.x, new_vec.y, new_vec.z);
	// 			// println!("VERTSSSSS {} {}", 
	// 			// 	(width - 0.5 * width) + ((seg2 - seg2 / 2) as f64) * (width - 0.5 * width) / (segments as f64),
	// 			// 	(width - 0.5 * width) + ((seg1 - seg1 / 2) as f64) * (width - 0.5 * width) / (segments as f64));
	// 			// vecs.push(Vector3d::new(
	// 			// 		0.0, 
	// 			// 		((seg1 - seg1 / 2) as f64) * (width - 0.5 * width) / (segments as f64),
	// 			// 		((seg2 - seg2 / 2) as f64) * (width - 0.5 * width) / (segments as f64) 
	// 			// 	)
	// 			// );


	// 		}
	// 		// println!("BLAH!! ++++++++++++++++++++++++++++++++++++++");
	// 	}

	// 	for seg1 in 0..(segments * segments) {
	// 		// println!("||||| {}", seg1);
	// 		if seg1 < (segments * segments - segments + 1) {
	// 			let mut new_e1 = [seg1 as i32, (seg1 + segments) as i32];
	// 			edges.push(new_e1);
	// 			println!("1 EDGE {} {} ", new_e1[0], new_e1[1]);// seg1, (seg1 + 1));
	// 		}
			
	// 		if ((seg1 + 0) % segments != 0 ) {
	// 			let mut new_e2 = [seg1 as i32, (seg1 + 1) as i32];
	// 			edges.push(new_e2);
	// 			println!("1 EDGE {} {} ", new_e2[0], new_e2[1]);// seg1, (seg1 + 1));
	// 		}
	// 		// println!("1 EDGE {} {}", seg1, (seg1 + segments));
	// 	}

	// 	// for seg2 in 0..segments {
	// 	// 	if seg2 < (seg2 * ) {
	// 	// 		edges.push([(seg2) as i32, (seg2 + 1) as i32]);
	// 	// 		// println!("2 EDGE {} {}", seg2, (seg2 + 1));
	// 	// 	}
	// 	// }

	// 	// println!("LENS {} {}", vecs[11].x, vecs[11].y);
	// 	// edges.push([0, 1]);
	// 	// edges.push([1, 2]);
	// 	// edges.push([2, 3]);
	// 	// edges.push([3, 0]);

	// 	// edges.push([0, 4]);
	// 	// edges.push([1, 5]);
	// 	// edges.push([2, 6]);
	// 	// edges.push([3, 7]);

	// 	// edges.push([4, 5]);
	// 	// edges.push([5, 6]);
	// 	// edges.push([6, 7]);
	// 	// edges.push([7, 4]);



	// 	// edges.push([0, 2]);
	// 	// edges.push([1, 3]);
		
	// 	// edges.push([4, 6]);
	// 	// edges.push([5, 7]);
	// 	let mut faces = vec![];

	// 	return Geometry::new(vecs, edges, faces);
	// }
}

#[cfg(test)]
mod tests {
	use super::*;
	// use super::super::paddle::{Paddle};

	// #[test]
}

