use byteorder::{NetworkEndian, WriteBytesExt};
use std::time::SystemTime;

/// Type representing the seed value
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Seed {
    value: u32,
}

impl Seed {
    /// Creates a new seed with a specific value
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    /// Creates a new seed based on the current time
    ///
    /// ```
    /// # use minimal_id::Seed;
    /// let seed = Seed::from_time();
    /// println!("Seed = {:?}", seed);
    /// ```
    pub fn from_time() -> Self {
        let year_start = SystemTime::UNIX_EPOCH;
        Self {
            value: get_seconds_since(year_start),
        }
    }

    /// Returns the seed as a byte array in Network endian
    ///
    /// This does make copies of the data, but since it's only 4 bytes
    /// we think it's an okay trade off.
    pub fn as_slice(&self) -> [u8; 4] {
        let mut data = vec![];
        data.write_u32::<NetworkEndian>(self.value).expect("Cannot write seed");

        let mut array = [0; 4];
        array.copy_from_slice(&data[..4]);
        array
    }
}

fn get_seconds_since(anchor: SystemTime) -> u32 {
    let duration = SystemTime::now()
        .duration_since(anchor)
        .expect("Unable to calculate duration");
    duration.as_secs() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functional_seed_to_slice() {
        let seed = Seed::new(10 << 24 | 20 << 16 | 30 << 8 | 40);
        let slc = seed.as_slice();
        assert_eq!(slc.get(0), Some(&10));
        assert_eq!(slc.get(1), Some(&20));
        assert_eq!(slc.get(2), Some(&30));
        assert_eq!(slc.get(3), Some(&40));
    }
}
