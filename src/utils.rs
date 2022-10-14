use crate::{Data, Hash};
use sha2::Digest;

pub(crate) fn hash_data(data: &Data) -> Hash {
    sha2::Sha256::digest(data).to_vec()
}

pub(crate) fn hash_concat(h1: &Hash, h2: &Hash) -> Hash {
    let h3 = h1.iter().chain(h2).copied().collect();
    hash_data(&h3)
}
