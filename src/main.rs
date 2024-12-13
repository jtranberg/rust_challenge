use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Account {
    id: String,
    balance: i64,
}

#[derive(Debug, Clone)]
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
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            chain: Vec::new(),
            pending_transactions: Vec::new(),
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
            let from_account = self.accounts.get(&from).unwrap();
            if from_account.balance >= amount {
                let transaction = Transaction {
                    from: Some(from.clone()),
                    to: to.clone(),
                    amount,
                };
                self.pending_transactions.push(transaction);
                println!("Transaction queued and will be confirmed in the next block.");
            } else {
                println!("Insufficient funds.");
            }
        } else {
            println!("One or both accounts not found.");
        }
    }

    fn get_balance(&self, id: &String) {
        match self.accounts.get(id) {
            Some(account) => println!("Balance of {}: {}", id, account.balance),
            None => println!("Account not found."),
        }
    }

    fn mint_block(&mut self) {
        if !self.pending_transactions.is_empty() {
            let block = Block {
                transactions: self.pending_transactions.drain(..).collect(),
                timestamp: current_timestamp(),
            };
            for tx in &block.transactions {
                if let Some(from) = &tx.from {
                    let mut from_account = self.accounts.get_mut(from).unwrap();
                    from_account.balance -= tx.amount;
                }
                let mut to_account = self.accounts.get_mut(&tx.to).unwrap();
                to_account.balance += tx.amount;
            }
            self.chain.push(block);
            println!("New block minted with confirmed transactions.");
        } else {
            println!("No transactions to confirm. Skipping block minting.");
        }
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn start_minting(blockchain: Arc<Mutex<Blockchain>>) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10));
        let mut blockchain = blockchain.lock().unwrap();
        blockchain.mint_block();
    });
}

fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    // Start block minting in a separate thread
    start_minting(Arc::clone(&blockchain));

    println!("Toy Blockchain 'B' is running...");
    loop {
        println!("Enter a command (create-account, transfer, balance, exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let args: Vec<&str> = input.trim().split_whitespace().collect();

        let mut blockchain = blockchain.lock().unwrap();

        match args.get(0) {
            Some(&"create-account") if args.len() == 3 => {
                let id = args[1].to_string();
                let balance = args[2].parse::<i64>().unwrap_or(0);
                blockchain.create_account(id, balance);
            }
            Some(&"transfer") if args.len() == 4 => {
                let from = args[1].to_string();
                let to = args[2].to_string();
                let amount = args[3].parse::<i64>().unwrap_or(0);
                blockchain.transfer(from, to, amount);
            }
            Some(&"balance") if args.len() == 2 => {
                let id = args[1].to_string();
                blockchain.get_balance(&id);
            }
            Some(&"exit") => break,
            _ => println!("Invalid command or arguments."),
        }
    }
}
