//! Proof abstraction and implementation
//!
//! A `Proof` consists of a path that on can follow from the first `Hash` in
//! the path to the root, with the `HashDirection` indicating whether
//! the `Hash` is on the left or on the right.
use crate::Hash;

#[derive(Debug, Default)]
pub struct Proof<'a> {
    /// The hashes to use when verifying the proof
    /// The first element of the tuple is which side the hash should be on when concatenating
    path: Vec<(HashDirection, &'a Hash)>,
}

impl<'a> Proof<'a> {
    pub(crate) fn new(path: Vec<(HashDirection, &'a Hash)>) -> Self {
        Self { path }
    }

    pub(crate) fn path(&self) -> &[(HashDirection, &'a Hash)] {
        &self.path
    }
}

/// Which side to put Hash on when concatenating proof hashes
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HashDirection {
    Left,
    Right,
}
