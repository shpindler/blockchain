pub struct Account {
    address: String,
    balance: u64,
}

impl Account {
    pub fn new(address: String, balance: u64) -> Account {
        Account { address, balance }
    }

    pub fn manage_account(&self) {
        println!("Address: {}, Balance: {}", self.address, self.balance);
    }

    pub fn transfer_nft(&mut self, _nft_id: String, _recipient: &mut Account) {
        // Logic to transfer NFT
        // Ensure you validate that `self` is the current owner of the NFT
    }
}