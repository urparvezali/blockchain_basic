use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Block {
    index: u32,
    data: String,
    time: u128,
    prev_hash: String,
    hash: String,
}
impl Block {
    pub fn new(index: u32, data: String, prev_hash: String) -> Self {
        let mut block = Block {
            index,
            data,
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            prev_hash,
            hash: String::new(),
        };
        block.hash = block.gen_hash();
        block
    }
    pub fn gen_hash(&self) -> String {
        let input_str = format!("{}{}{}{}", self.index, self.data, self.time, self.prev_hash);
        let mut hasher = Sha256::new();
        hasher.update(input_str.as_bytes());
        let hash = hasher.finalize();
        format!("{:x}", hash)
    }
}

pub struct BlockChain {
    chain: Vec<Block>,
}
impl BlockChain {
    pub fn new() -> Self {
        Self {
            chain: vec![Block::new(0, "Genesis".to_string(), "0".to_string())],
        }
    }
    pub fn add(&mut self, data: String) {
        self.chain.push(Block::new(
            self.chain.last().unwrap().index + 1,
            data,
            self.chain.last().unwrap().hash.clone(),
        ));
    }
    pub fn display(&self) {
        for block in &self.chain {
            println!("{:?}", block);
        }
    }
}
