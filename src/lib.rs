#[derive(PartialEq, Debug)]
pub struct Generator {}
#[derive(PartialEq, Debug)]
pub struct MinimalId {}

impl Default for Generator {
	fn default() -> Self { Generator {} }
}

impl Default for MinimalId {
	fn default() -> Self { MinimalId {} }
}

impl Generator {
	pub fn generate(&self) -> MinimalId { MinimalId::default() }
	pub fn id_from_str(&self, _id_str: &str) -> MinimalId { MinimalId::default() }
}

impl MinimalId {
	pub fn to_string(&self) -> String { "".to_string() }

	pub fn to_slice(&self) -> &[u8] { &[0] }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn acceptance_test_round_trip() {
		let generator = Generator::default();
		let id = generator.generate();
		let id_str = id.to_string();
		let id_int = id.to_slice();
		let actual = generator.id_from_str(&id_str);
		let actual_int = actual.to_slice();
		assert_eq!(id, actual);
		assert_eq!(id_int, actual_int);
	}
}
