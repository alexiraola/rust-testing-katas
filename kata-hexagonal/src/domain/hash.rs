use sha2::{Digest, Sha256};

pub fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result) // Convert the hash result to a hex string
}
