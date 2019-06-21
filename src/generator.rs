use super::MinimalId;
use super::Seed;

/// # Generator
///
/// top level structure for interfacing with the library
///
/// ```
/// use minimal_id::Generator;
/// let generator = Generator::default();
/// ```
#[derive(PartialEq, Debug)]
pub struct Generator {}

impl Default for Generator {
	/// Creates the generator
	///
	/// This doesn't really do anything other than contain the functions
	/// This can probably be deleted.
	fn default() -> Self { Generator {} }
}

impl Generator {
	/// Returns a new Minimal Id
	pub fn generate(&self) -> MinimalId {
		let seed = Seed::from_time();
		MinimalId::new(&seed)
	}

	/// Parse a string into a minimal ID
	///
	/// ```
	/// # use minimal_id::*;
	/// # let generator = Generator::default();
	/// let id = generator
	/// 	.id_from_str("AAECAwQFBgcI")
	/// 	.expect("Cannot parse String into ID");
	/// assert_eq!(id.to_slice()[0], 0);
	/// ```
	// TODO(#3): Improve Error Handling
	pub fn id_from_str(&self, id_str: &str) -> Result<MinimalId, ()> {
		MinimalId::id_from_str(id_str)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn functional_test_generate_unique_ids() {
		let generator = Generator::default();
		let id1 = generator.generate();
		let id2 = generator.generate();
		assert_ne!(id1, id2);
	}
}
