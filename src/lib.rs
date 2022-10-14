use crate::{node::Node, proof::Proof};

mod node;
mod proof;
mod utils;

pub type Data = Vec<u8>;
pub type Hash = Vec<u8>;

pub struct MerkleTree {
    /// Full tree data
    data: Vec<Node>,
    /// Amount of rows in the tree excluding the root Hash row
    /// depth = log2(data.len)
    depth: usize,
    /// Cached root of the tree
    root: Hash,
}

impl MerkleTree {
    /// Gets root hash for this tree
    pub fn root(&self) -> Hash {
        todo!("For tests to work")
    }

    /// Constructs a Merkle tree from given input data
    pub fn construct(input: &[Data]) -> MerkleTree {
        todo!("Exercise 1")
    }

    /// Verifies that the given input data produces the given root hash
    pub fn verify(input: &[Data], root_hash: &Hash) -> bool {
        todo!("Exercise 1b")
    }

    /// Verifies that the given data and proof_path correctly produce the given root_hash
    pub fn verify_proof(data: &Data, proof: &Proof, root_hash: &Hash) -> bool {
        todo!("Exercise 2")
    }

    /// Returns a list of hashes that can be used to prove that the given data is in this tree
    pub fn prove(&self, data: &Data) -> Option<Proof> {
        todo!("Exercise 3")
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    fn example_data(n: usize) -> Vec<Data> {
        let mut data = vec![];
        for i in 0..n {
            data.push(vec![i as u8]);
        }
        data
    }

    #[test]
    fn test_constructions() {
        let data = example_data(4);
        let tree = MerkleTree::construct(&data);
        let expected_root = "9675e04b4ba9dc81b06e81731e2d21caa2c95557a85dcfa3fff70c9ff0f30b2e";
        assert_eq!(hex::encode(tree.root()), expected_root);

        let data = example_data(3);
        let tree = MerkleTree::construct(&data);
        let expected_root = "773a93ac37ea78b3f14ac31872c83886b0a0f1fec562c4e848e023c889c2ce9f";
        assert_eq!(hex::encode(tree.root()), expected_root);

        let data = example_data(8);
        let tree = MerkleTree::construct(&data);
        let expected_root = "0727b310f87099c1ba2ec0ba408def82c308237c8577f0bdfd2643e9cc6b7578";
        assert_eq!(hex::encode(tree.root()), expected_root);
    }
}
