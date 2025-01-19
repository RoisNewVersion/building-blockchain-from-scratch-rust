use std::time::SystemTime;

#[derive(Debug)]
struct Block {
    nonce: i32,
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
    fn new (nonce: i32, previous_hash:Vec<u8>) -> Self {
        // the method will take control of the input of previous_hash
        let time_now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        Block{
            nonce: nonce,
            previous_hash: previous_hash,
            transactions: Vec::<Vec<u8>>::new(),
            time_stamp: time_now.as_nanos(),
        } // don't add semicolon because we want to return this object
    }

    // The second is Struct method which is need to read or write to fields of the struct.
    // self which reference to the struct instance.
    fn print(&self) {
        // format value as hex
        println!("time_stamp: {:x}", self.time_stamp);
        // format integer
        println!("nonce: {}", self.nonce);
        // format vector, ask the compiler to do it.
        println!("previous_hash: {:?}", self.previous_hash);
        println!("transactions: {:?}", self.transactions);
    }
}

fn main() {
    // convert a string into bytes array
    // convert it to String, into_bytes() => Vec<u8>
    let b = Block::new(0, "this is out first block!".to_string().into_bytes());
    b.print();
    // 
    println!("the first block is : {:?}", b);
}
