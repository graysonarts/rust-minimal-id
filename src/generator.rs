use super::MinimalId;
use super::Seed;

/// # Generator
///
/// top level structure for interfacing with the library
///
/// ```
/// use minimal_id::Generator;
/// let id = Generator::new_id();
/// ```
#[derive(PartialEq, Debug)]
pub struct Generator {}

impl Default for Generator {
    /// Creates the generator
    ///
    /// This doesn't really do anything other than contain the functions
    /// This can probably be deleted.
    fn default() -> Self {
        Generator {}
    }
}

impl Generator {
    /// Returns a new Minimal Id
    #[deprecated(since = "0.8.0", note = "Use Generator::new_id or MinimalId::generate")]
    pub fn generate(&self) -> MinimalId {
        Generator::new_id()
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

    pub fn new_id() -> MinimalId {
        let seed = Seed::from_time();
        MinimalId::new(&seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functional_test_generate_unique_ids() {
        let id1 = Generator::new_id();
        let id2 = Generator::new_id();
        assert_ne!(id1, id2);
    }
}
