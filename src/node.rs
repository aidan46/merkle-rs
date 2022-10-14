//! Node abstraction and implementation.
//!
//! A node is one element of the `MerkleTree`.
//! Internally the `Node`'s are aware of their parents in order to
//! implement getting the `Proof`
use crate::Hash;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Node {
    hash: Hash,
    // Only `None` for root node
    parent: Option<usize>,
}

impl Node {
    pub(super) fn new(hash: Hash, parent: Option<usize>) -> Self {
        Self { hash, parent }
    }

    pub(super) fn get_hash(&self) -> &Hash {
        &self.hash
    }

    pub(super) fn get_parent(&self) -> Option<usize> {
        self.parent
    }

    pub(super) fn set_parent(&mut self, parent: usize) {
        self.parent = Some(parent);
    }
}
