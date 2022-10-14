use crate::{
    node::Node,
    proof::Proof,
    utils::{hash_concat, hash_data, log2},
};

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
        self.root.clone()
    }

    /// Constructs a Merkle tree from given input data
    /// # Panics
    /// We assume that the length of the input is a power of 2 (perfect binary tree)
    /// so we assert in the beginning that this is true
    pub fn construct(input: &[Data]) -> MerkleTree {
        // Assert because assumption is made that input is a perfect binary tree
        assert!(input.len().is_power_of_two());
        // Create all the hashed leaf nodes
        let mut data: Vec<Node> = input
            .iter()
            .map(|x| Node::new(hash_data(x), None))
            .collect();
        let mut width = input.len();
        let depth = log2(width);

        // Keep added hashes until 2 child nodes are left, arriving at the root
        while width > 1 {
            // Build layer above current on
            let end = data.len();
            let start = end - width;
            for i in (start..end).step_by(2) {
                let parent_node = Self::create_parent(&mut data, i);
                data.push(parent_node);
            }
            // Subtract one from exponent
            width >>= 1;
        }

        let root = data[data.len() - 1].get_hash().clone();
        Self { data, depth, root }
    }

    /// Verifies that the given input data produces the given root hash
    pub fn verify(input: &[Data], root_hash: &Hash) -> bool {
        &MerkleTree::construct(input).root() == root_hash
    }

    /// Verifies that the given data and proof_path correctly produce the given root_hash
    pub fn verify_proof(data: &Data, proof: &Proof, root_hash: &Hash) -> bool {
        todo!("Exercise 2")
    }

    /// Returns a list of hashes that can be used to prove that the given data is in this tree
    pub fn prove(&self, data: &Data) -> Option<Proof> {
        todo!("Exercise 3")
    }

    fn create_parent(data: &mut [Node], i: usize) -> Node {
        let left = &data[i];
        let right = &data[i + 1];
        let hash = hash_concat(left.get_hash(), right.get_hash());
        let parent = data.len();
        data[i].set_parent(parent);
        data[i + 1].set_parent(parent);
        Node::new(hash, None)
    }
}

#[cfg(test)]
mod tests {
    use super::{Data, MerkleTree};

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

        let data = example_data(8);
        let tree = MerkleTree::construct(&data);
        let expected_root = "0727b310f87099c1ba2ec0ba408def82c308237c8577f0bdfd2643e9cc6b7578";
        assert_eq!(hex::encode(tree.root()), expected_root);
    }

    #[test]
    fn test_verify_root() {
        let data = example_data(8);
        let tree = MerkleTree::construct(&data);
        let root = tree.root();
        let expected_root = "0727b310f87099c1ba2ec0ba408def82c308237c8577f0bdfd2643e9cc6b7578";
        assert!(MerkleTree::verify(&data, &root));
        assert_eq!(hex::encode(root), expected_root);
    }
}
