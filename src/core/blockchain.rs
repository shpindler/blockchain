use super::block::Block;
use crate::runtime::transaction::{Transaction, TransactionData};
use std::collections::HashMap;

fn now() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration_since_epoch.as_millis()
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub current_transactions: Vec<Transaction>,
    pub accounts: HashMap<String, f32>,
    pub reward: f32,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            chain: vec![Block::new(0, 0, "Genesis block".to_string(), String::new())],
            current_transactions: vec![],
            accounts: HashMap::new(),
            reward: 0.0,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(self.chain.len() as u64, now(), data, previous_block.hash.clone());
        self.chain.push(new_block);
    }

    pub fn create_transaction(&mut self, sender: String, receiver: String, amount: f32, data: TransactionData) -> bool {
        if self.accounts.get(&sender).unwrap_or(&0.0) < &amount {
            return false; // Недостаточно средств
        }

        let transaction = Transaction { sender, receiver, amount, data: Some(data) };
        self.current_transactions.push(transaction);
        true
    }

    fn update_state(&mut self) {
        for transaction in &self.current_transactions {
            let sender_balance = self.accounts.entry(transaction.sender.clone()).or_insert(0.0);
            *sender_balance -= transaction.amount;

            let receiver_balance = self.accounts.entry(transaction.receiver.clone()).or_insert(0.0);
            *receiver_balance += transaction.amount;
        }
        self.current_transactions.clear();
    }

    pub fn mine_block(&mut self, miner_address: String) {
        // Создаем новый блок с текущими транзакциями
        let mut block = Block {
            index: self.chain.len() as u64,
            timestamp: now(),
            transactions: self.current_transactions.clone(),
            previous_hash: if let Some(block) = self.chain.last() {
                block.hash.clone()
            } else {
                String::new()
            },
            hash: String::new(),
            data: String::new(),
        };

        // Вызываем метод для обновления состояния аккаунтов
        self.update_state();

        // Добавляем награду для майнера
        let reward_transaction = Transaction {
            sender: "0".to_string(),
            receiver: miner_address,
            amount: self.reward,
            data: None,
        };
        self.current_transactions.push(reward_transaction);

        // После того как транзакции добавлены и состояние обновлено, 
        // мы можем вычислить хеш блока
        block.hash = block.calculate_hash();

        // Добавляем блок в цепочку
        self.chain.push(block);

        // Очищаем список текущих транзакций, так как они уже включены в блок
        self.current_transactions.clear();
    }
}