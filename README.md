# 🌲 Merkle-rs
A simple implementation of a Merkle Tree

## Usage
```rust
use merkle_rs::{Data, MerkleTree};

fn example_data(n: usize) -> Vec<Data> {
    let mut data = vec![];
    for i in 0..n {
        data.push(vec![i as u8]);
    }
    data
}

fn main() {
    let data = example_data(16);
    let tree = MerkleTree::construct(&data);
    if let Some(proof) = tree.prove(&vec![2]) {
        for step in proof.path() {
            println!("Direction: {:?} Hash: {}", step.0, hex::encode(step.1));
        }
    }
}
```

## Details
A merkle tree is a data structure used for data verification and synchronization.
It is a tree data structure where each non-leaf node is a hash of it’s child nodes.
All the leaf nodes are at the same depth and are as far left as possible.
![MerkleTree](https://miro.medium.com/max/1200/0*jY57ovz5FRz05Svg.png)

### MerkleTree
```rust
struct MerkleTree {
	/// Full tree data
	data: Vec<Node>,
	/// Amount of rows in the tree
	/// row_count = log2(len)
	row_count: usize,
	/// Cached root of the tree
	root: Hash,
}

type Data = Vec<u8>;
type Hash = Vec<u8>;
```
The `MerkleTree` takes as input an array of `Data` where the length of this array is a power of 2.
During construction the array is hashed into `Hash` and a tree is created. The relationship between the length of the input array (`l`) and the amount of final nodes (`n`) is: 
```text
if l = 2^x then n = 2^(x + 1) - 1
```
The constructed `MerkleTree` can prove whether or not a certain `Data` is in the tree.
It obtains checks if the `Hash` of the input is included in the tree and then it constructs a `Proof`.

### Node
```rust
struct Node {
	hash: Hash,
	/// Only `None` for root node
	parent: Option<usize>,
}
```
The `Node` struct represents a single node in the tree. It has methods implemented to get and set its parent.

### Proof
```rust
struct Proof {
	hashes: Vec<(HashDirection, &'a Hash)>,
}

enum HashDirection {
	Left,
	Right,
}
```
The `Proof` is a list to get from the given data to the root hash. The first element of the tuple is which side the hash should be on when concatenating. The second element is a reference to the hashed value of the current node.
