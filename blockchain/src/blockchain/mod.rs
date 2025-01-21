use std::time::SystemTime;

use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    pub nonce: i32,
    previous_hash: Vec<u8>,
    time_stamp: u128,
    transactions: Vec<Vec<u8>>,
}

/*
CamelCase for name of Struct, snake_case for name of fields
*/

impl Block {
    // method for the struct, class methods.
    // Two kinds of method, one kind static method which not reading or writing into fields of the block.
    // Self is alias name for Object, if we change the name of the Struct then we don't need to change the name inside.
    pub fn new(nonce: i32, previous_hash: Vec<u8>) -> Self {
        // the method will take control of the input of previous_hash
        let time_now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        Block {
            nonce: nonce,
            previous_hash: previous_hash,
            transactions: Vec::<Vec<u8>>::new(),
            time_stamp: time_now.as_nanos(),
        } // don't add semicolon because we want to return this object
    }

    // The second is Struct method which is need to read or write to fields of the struct.
    // self which reference to the struct instance.
    pub fn print(&self) {
        // format value as hex
        println!("time_stamp: {:x}", self.time_stamp);
        // format integer
        println!("nonce: {}", self.nonce);
        // format vector, ask the compiler to do it.
        println!("previous_hash: {:?}", self.previous_hash);
        println!("transactions: {:?}", self.transactions);
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut bin = Vec::<u8>::new();
        bin.extend(self.nonce.to_be_bytes());
        bin.extend(self.previous_hash.clone());
        bin.extend(self.time_stamp.to_be_bytes());
        for tx in self.transactions.iter() {
            bin.extend(tx.clone());
        }
        let mut hasher = Sha256::new();
        hasher.update(bin);
        hasher.finalize().to_vec()
    }
}

#[derive(Debug)]
pub struct BlockChain {
    transaction_pool: Vec<Vec<u8>>,
    chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut bc = BlockChain {
            transaction_pool: Vec::<Vec<u8>>::new(),
            chain: Vec::<Block>::new(),
        };

        bc.create_block(0, vec![0 as u8; 32]);
        bc // no semicolon
    }

    pub fn create_block(&mut self, nonce: i32, previous_hash: Vec<u8>) {
        let b = Block::new(nonce, previous_hash);
        self.chain.push(b)
    }

    pub fn print(&self) {
        for (i, block) in self.chain.iter().enumerate() {
            println!("{} Chain {} {}", "=".repeat(25), i, "=".repeat(25));
            block.print();
        }
        println!("{}", "*".repeat(25));
    }

    pub fn last_block(&self) -> &Block {
        if self.chain.len() > 1 {
            return &self.chain[self.chain.len() - 1];
        }
        &self.chain[0]
    }
}
