// use std::fs::File;
// use std::io::Read;
// use std::io::{self, BufReader};

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
// use std::path::Path;

use super::vector3d::Vector3d;
use super::geometry::Geometry;
use super::face::Face;
use super::mesh::Mesh;

pub struct Importer;

impl Importer {
	pub fn import_obj(file_path: &str) -> Mesh {
		let f = File::open(file_path).unwrap();
	    let file = BufReader::new(&f);

	    let mut vertices: Vec<Vector3d> = Vec::new();
	    let mut faces: Vec<Face> = Vec::new();

	    for line in file.lines() {
	        let l = line.unwrap();

	        if &l[0..1] == "v" {

	        	let split: Vec<&str> = l.split(" ").collect();
	        	let x: f64 = split[1].parse().unwrap();
	        	let y: f64 = split[2].parse().unwrap();
	        	let z: f64 = split[3].parse().unwrap();

	        	vertices.push(Vector3d::new(
	        		x, 
	        		y, 
	        		z));

	        } else 
	        if &l[0..1] == "f" {

	        	let split: Vec<&str> = l.split(" ").collect();
	        	let a: i32 = split[1].parse().unwrap();
	        	let b: i32 = split[2].parse().unwrap();
	        	let c: i32 = split[3].parse().unwrap();

	        	faces.push(Face::new(
	        		(a - 1) as usize, 
	        		(b - 1) as usize, 
	        		(c - 1) as usize));

	        }
	        // println!("{:?}", &l[0..1] == "f"); 
	    }

	    let geo = Geometry::new(vertices, faces);
	    let mesh = Mesh::new(geo);
	    return mesh;
		// let mut data = String::new();
	 //    let mut f = File::open(file_path);//.unwrap();

	 //    // Collect all lines into a vector
	 //    let f = BufReader::new(f);
	 //    for l in f.lines() {
	 //    	println!("{}", "hello");
	 //    }


	    // let lines: Vec<_> = reader.lines();//.collect();
	    // println!("{:?}", lines.len());
	    // f.read_to_string(&mut data).unwrap();
	    // println!("{}", data);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	// use super::super::paddle::{Paddle};

	// #[test]
}