pub struct Face {
	pub a: usize,
	pub b: usize,
	pub c: usize
}

impl Face {
	pub fn new(a: usize, b: usize, c: usize) -> Face {
		Face{
			a: a,
			b: b,
			c: c
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	// use super::super::paddle::{Paddle};

	// #[test]
}