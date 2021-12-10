use sha2::{Digest, Sha256};
use std::fmt;
use std::time;

fn to_hex_string(bytes: &[u8]) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join("")
}
pub struct Block {
    timestamp: u64,
    data: Vec<u8>,
    previous_hash: [u8; 32],
    pub hash: [u8; 32],
}

impl Block {
    fn set_hash(&mut self) {
        let mut sha = Sha256::new();
        sha.update(&self.previous_hash);
        sha.update(&self.data);
        sha.update(&self.timestamp.to_le_bytes());
        self.hash.copy_from_slice(&sha.finalize());
    }

    pub fn new(data: Vec<u8>, previous_hash: &[u8; 32]) -> Block {
        let mut block = Block {
            timestamp: time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data,
            previous_hash: *previous_hash,
            hash: [0; 32],
        };
        block.set_hash();
        block
    }

    pub fn new_genesis() -> Block {
        Block::new("Genesis Block".as_bytes().to_vec(), &[0; 32])
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pad = " ".repeat(9);
        f.write_str(&format!(
            "Block(\n  previous_hash: {},\n  {}hash: {},\n  {}data: {},\n)",
            to_hex_string(&self.previous_hash),
            pad,
            to_hex_string(&self.hash),
            pad,
            String::from_utf8_lossy(&self.data),
        ))
    }
}
