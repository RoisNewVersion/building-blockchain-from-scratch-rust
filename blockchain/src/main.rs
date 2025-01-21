pub mod blockchain;
use blockchain::BlockChain;
use sha2::{Digest, Sha256};
fn main() {
    let mut hasher = Sha256::new();
    hasher.update(b"hello world\n");
    let result = hasher.finalize();
    println!("The result is : {:x}", result);
    // convert a string into bytes array
    // convert it to String, into_bytes() => Vec<u8>
    // let b = Block::new(0, "this is out first block!".to_string().into_bytes());
    // b.print();
    // println!("the first block is : {:?}", b);

    let mut block_chain = BlockChain::new();
    println!("Block chain: {:?}", block_chain);

    block_chain.print();

    let previous_hash = block_chain.last_block().hash();
    block_chain.create_block(1, previous_hash);
    block_chain.print();

    let previous_hash = block_chain.last_block().hash();
    block_chain.create_block(2, previous_hash);
    block_chain.print();
}
