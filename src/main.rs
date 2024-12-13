use std::collections::HashMap;

#[derive(Debug)]
struct Account {
    id: String,
    balance: i64,
}

#[derive(Debug)]
struct Transaction {
    from: Option<String>,
    to: String,
    amount: i64,
}

#[derive(Debug)]
struct Block {
    transactions: Vec<Transaction>,
    timestamp: u64,
}

#[derive(Debug)]
struct Blockchain {
    accounts: HashMap<String, Account>,
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            chain: Vec::new(),
        }
    }

    fn create_account(&mut self, id: String, balance: i64) {
        if self.accounts.contains_key(&id) {
            println!("Account already exists!");
        } else {
            self.accounts.insert(
                id.clone(),
                Account {
                    id: id.clone(),
                    balance,
                },
            );
            println!("Account created successfully with id: {}", id);
        }
    }

    fn transfer(&mut self, from: String, to: String, amount: i64) {
        if self.accounts.contains_key(&from) && self.accounts.contains_key(&to) {
            let mut from_account = self.accounts.remove(&from).unwrap();
            let mut to_account = self.accounts.remove(&to).unwrap();

            if from_account.balance >= amount {
                from_account.balance -= amount;
                to_account.balance += amount;

                let transaction = Transaction {
                    from: Some(from.clone()),
                    to: to.clone(),
                    amount,
                };
                println!("Created transaction: {:?}", transaction);

                self.add_transaction(transaction);

                println!("Transfer successful.");
            } else {
                println!("Insufficient funds.");
            }

            self.accounts.insert(from.clone(), from_account);
            self.accounts.insert(to.clone(), to_account);
        } else {
            println!("One or both accounts not found.");
        }
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        let block = Block {
            transactions: vec![transaction],
            timestamp: 123456789, // Replace with actual timestamp
        };
        self.chain.push(block);
        println!("Added block to chain: {:?}", self.chain.last());
    }

    fn get_balance(&self, id: &String) {
        match self.accounts.get(id) {
            Some(account) => println!("Balance of {}: {}", id, account.balance),
            None => println!("Account not found."),
        }
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    // Example usage
    blockchain.create_account("Alice".to_string(), 100);
    blockchain.create_account("Bob".to_string(), 50);

    blockchain.get_balance(&"Alice".to_string());
    blockchain.get_balance(&"Bob".to_string());

    blockchain.transfer("Alice".to_string(), "Bob".to_string(), 30);

    blockchain.get_balance(&"Alice".to_string());
    blockchain.get_balance(&"Bob".to_string());
}
