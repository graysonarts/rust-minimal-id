use byteorder::{NetworkEndian, WriteBytesExt};

pub struct Seed {
	value: u32,
}

impl Seed {
	pub fn new(value: u32) -> Self { Self { value } }

	pub fn from_time() -> Self { Self { value: 0 } }

	pub fn as_slice(&self) -> [u8; 4] {
		// TODO(#1): Force to Network Byte Order
		unsafe { std::mem::transmute::<u32, [u8; 4]>(self.value) }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn functional_seed_to_slice() {
		let seed = Seed::new(10 << 24 | 20 << 16 | 30 << 8 | 40);
		let slc = seed.as_slice();
		assert_eq!(slc.get(3), Some(&10));
		assert_eq!(slc.get(2), Some(&20));
		assert_eq!(slc.get(1), Some(&30));
		assert_eq!(slc.get(0), Some(&40));
	}
}
