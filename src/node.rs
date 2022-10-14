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
