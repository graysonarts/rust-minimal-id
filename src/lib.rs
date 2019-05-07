mod seed;

pub use seed::Seed;
use rand::prelude::*;
use data_encoding::BASE64URL_NOPAD;

const ID_SIZE : usize = 9;

#[derive(PartialEq, Debug)]
pub struct Generator {}

pub struct MinimalId {
	value: [u8; ID_SIZE]
}

impl Default for Generator {
	fn default() -> Self { Generator {} }
}

impl Default for MinimalId {
	fn default() -> Self {
		MinimalId {
			value: [0; ID_SIZE]
		}
	}
}

impl PartialEq for MinimalId {
	fn eq(&self, rhs: &Self) -> bool {
		self.value.iter().zip(rhs.value.iter()).all(|(x, y)| x == y)
	}
}

impl std::fmt::Debug for MinimalId {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		self.to_slice().iter().for_each(|x| write!(f, "{:?}-", x).unwrap());
		Ok(())
	}
}

impl Generator {
	pub fn generate(&self) -> MinimalId {
		let seed = Seed::new(0);
		MinimalId::new(&seed)
	}
	pub fn id_from_str(&self, id_str: &str) -> Result<MinimalId, ()> {
		let value = BASE64URL_NOPAD.decode(id_str.as_bytes()).map_err(|_| ())?;
		Ok(MinimalId::from_slice(&value))
	 }
}

impl MinimalId {
	pub fn to_string(&self) -> String {
		BASE64URL_NOPAD.encode(&self.value)
	 }

	pub fn to_slice(&self) -> &[u8] { &self.value }

	fn new(seed: &Seed) -> Self {
		let mut rng = [0u8; ID_SIZE-4];
		rand::thread_rng().fill_bytes(&mut rng);
		let mut vec = Vec::with_capacity(ID_SIZE);
		vec.extend_from_slice(&seed.as_slice());
		vec.extend_from_slice(&rng);
		let mut value : [u8; ID_SIZE] = [0; ID_SIZE];
		value.copy_from_slice(vec.as_slice());

		Self {
			value
		}
	}

	fn from_bytes(buf: [u8; ID_SIZE]) -> Self {
		Self {
			value: buf
		}
	}

	fn from_slice(buf: &[u8]) -> Self {
		let mut data : [u8; 9] = [0; 9];
		data.copy_from_slice(buf);
		Self {
			value: data
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test(ignore=true)]
	fn acceptance_test_round_trip() {
		let generator = Generator::default();
		let id = generator.generate();
		let id_str = id.to_string();
		let actual = generator.id_from_str(&id_str).expect("Unable to parse id string");
		assert_eq!(id, actual);
	}

	#[test]
	fn functional_serializes_to_encoded_string() {
		let id = MinimalId::from_bytes([
			0, 1, 2, 3, 4, 5, 6, 7, 8
		]);
		let expected_encoding = "AAECAwQFBgcI";

		assert_eq!(id.to_string(), expected_encoding);
	}

	#[test]
	fn functional_test_generate_unique_ids() {
		let generator = Generator::default();
		let id1 = generator.generate();
		let id2 = generator.generate();
		assert_ne!(id1, id2);
	}
}
