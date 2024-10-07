use blockchain_basic::BlockChain;

fn main() {
    let mut blockchain = BlockChain::new();

    blockchain.add("First Block after Genesis".to_string());
    blockchain.add("Second Block after Genesis".to_string());

    blockchain.display();
}
