extern crate shpindler_blockchain;
use shpindler_blockchain::core::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("First block data".to_string());
    blockchain.add_block("Second block data".to_string());

    for block in blockchain.chain {
        println!("{:?}", block);
    }
}
