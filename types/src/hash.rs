use std::{convert::TryFrom, fmt};

use rand::{rngs::OsRng, Rng};

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct HashValue {
    hash: [u8; HashValue::LENGTH],
}

impl HashValue {
    /// The length of the hash in bytes.
    pub const LENGTH: usize = 32;
    /// The length of the hash in bits.
    pub const LENGTH_IN_BITS: usize = Self::LENGTH * 8;

    /// Create a new [`HashValue`] from a byte array.
    pub fn new(hash: [u8; HashValue::LENGTH]) -> Self {
        HashValue { hash }
    }

    /// Create from a slice (e.g. retrieved from storage).
    pub fn from_slice<T: AsRef<[u8]>>(bytes: T) -> Result<Self, HashValueParseError> {
        <[u8; Self::LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| HashValueParseError)
            .map(Self::new)
    }

    /// Dumps into a vector.
    pub fn to_vec(&self) -> Vec<u8> {
        self.hash.to_vec()
    }

    /// Creates a zero-initialized instance.
    pub const fn zero() -> Self {
        HashValue {
            hash: [0; HashValue::LENGTH],
        }
    }

    /// Create a cryptographically random instance.
    pub fn random() -> Self {
        let mut rng = OsRng;
        let hash: [u8; HashValue::LENGTH] = rng.gen();
        HashValue { hash }
    }

    /// Creates a random instance with given rng. Useful in unit tests.
    pub fn random_with_rng<R: Rng>(rng: &mut R) -> Self {
        let hash: [u8; HashValue::LENGTH] = rng.gen();
        HashValue { hash }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HashValueParseError;

impl fmt::Display for HashValueParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to parse HashValue")
    }
}

impl std::error::Error for HashValueParseError {}
