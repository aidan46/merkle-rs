use crate::{Data, Hash};
use sha2::Digest;

pub(crate) fn hash_data(data: &Data) -> Hash {
    sha2::Sha256::digest(data).to_vec()
}

pub(crate) fn hash_concat(h1: &Hash, h2: &Hash) -> Hash {
    let h3 = h1.iter().chain(h2).copied().collect();
    hash_data(&h3)
}

pub(crate) fn log2(n: usize) -> usize {
    n.trailing_zeros() as usize
}

pub(crate) fn is_even(n: usize) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::log2;

    #[test]
    fn test_log2() {
        assert_eq!(log2(1), 0);
        assert_eq!(log2(2), 1);
        assert_eq!(log2(4), 2);
        assert_eq!(log2(8), 3);
        assert_eq!(log2(16), 4);
    }
}
