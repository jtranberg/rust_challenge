use std::collections::HashMap; // For storing accounts using a HashMap
use std::io; // For input and output
use std::sync::{Arc, Mutex}; // For thread-safe shared access
use std::thread; // For spawning threads
use std::time::{Duration, SystemTime, UNIX_EPOCH}; // For handling time

// Represents a user account with an ID and balance
#[derive(Debug)]
struct Account {
    id: String, // Unique identifier for the account
    balance: i64, // Current balance of the account
}

// Represents a transaction in the blockchain
#[derive(Debug, Clone)]
struct Transaction {
    from: Option<String>, // Sender's account ID (None for system-generated transactions)
    to: String,           // Receiver's account ID
    amount: i64,          // Amount to transfer
}

// Represents a block in the blockchain
#[derive(Debug)]
struct Block {
    transactions: Vec<Transaction>, // List of transactions in the block
    timestamp: u64,                 // Time when the block was created
}

// Represents the entire blockchain
#[derive(Debug)]
struct Blockchain {
    accounts: HashMap<String, Account>, // Map of account IDs to Account structs
    chain: Vec<Block>,                  // List of all blocks in the blockchain
    pending_transactions: Vec<Transaction>, // Transactions waiting to be included in a block
}

impl Blockchain {
    // Creates a new, empty blockchain
    fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            chain: Vec::new(),
            pending_transactions: Vec::new(),
        }
    }

    // Adds a new account to the blockchain
    fn create_account(&mut self, id: String, balance: i64) {
        if self.accounts.contains_key(&id) {
            println!("Account already exists!"); // Error if the account already exists
        } else {
            self.accounts.insert(
                id.clone(),
                Account {
                    id: id.clone(),
                    balance,
                },
            );
            println!("Account created successfully with id: {}", id); // Success message
        }
    }

    // Queues a transaction to transfer funds between accounts
    fn transfer(&mut self, from: String, to: String, amount: i64) {
        if self.accounts.contains_key(&from) && self.accounts.contains_key(&to) {
            let from_account = self.accounts.get(&from).unwrap();
            if from_account.balance >= amount {
                let transaction = Transaction {
                    from: Some(from.clone()),
                    to: to.clone(),
                    amount,
                };
                self.pending_transactions.push(transaction); // Add transaction to pending queue
                println!("Transaction queued and will be confirmed in the next block.");
            } else {
                println!("Insufficient funds."); // Error for insufficient balance
            }
        } else {
            println!("One or both accounts not found."); // Error for missing accounts
        }
    }

    // Retrieves and displays the balance of a specific account
    fn get_balance(&self, id: &String) {
        match self.accounts.get(id) {
            Some(account) => println!("Balance of {}: {}", id, account.balance), // Display balance
            None => println!("Account not found."), // Error for nonexistent account
        }
    }

    // Mints a new block and processes pending transactions
    fn mint_block(&mut self) {
        if !self.pending_transactions.is_empty() {
            let block = Block {
                transactions: self.pending_transactions.drain(..).collect(), // Move transactions to block
                timestamp: current_timestamp(), // Use current time as block timestamp
            };
            // Process transactions in the block
            for tx in &block.transactions {
                if let Some(from) = &tx.from {
                    let mut from_account = self.accounts.get_mut(from).unwrap();
                    from_account.balance -= tx.amount; // Deduct amount from sender
                }
                let mut to_account = self.accounts.get_mut(&tx.to).unwrap();
                to_account.balance += tx.amount; // Add amount to receiver
            }
            self.chain.push(block); // Add the new block to the chain
            println!("New block minted with confirmed transactions.");
        } else {
            println!("No transactions to confirm. Skipping block minting.");
        }
    }
}

// Returns the current timestamp in seconds since UNIX_EPOCH
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// Starts a separate thread to mint blocks every 10 seconds
fn start_minting(blockchain: Arc<Mutex<Blockchain>>) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10)); // Wait 10 seconds
        let mut blockchain = blockchain.lock().unwrap(); // Lock the blockchain for safe access
        blockchain.mint_block(); // Mint a new block
    });
}

// Main function to run the blockchain simulation
fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new())); // Create a new blockchain

    // Start block minting in a background thread
    start_minting(Arc::clone(&blockchain));

    println!("Toy Blockchain 'B' is running...");
    loop {
        // Prompt the user for input
        println!("Enter a command (create-account, transfer, balance, exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let args: Vec<&str> = input.trim().split_whitespace().collect();

        let mut blockchain = blockchain.lock().unwrap(); // Lock the blockchain for safe access

        // Handle user commands
        match args.get(0) {
            Some(&"create-account") if args.len() == 3 => {
                let id = args[1].to_string();
                let balance = args[2].parse::<i64>().unwrap_or(0);
                blockchain.create_account(id, balance); // Create an account
            }
            Some(&"transfer") if args.len() == 4 => {
                let from = args[1].to_string();
                let to = args[2].to_string();
                let amount = args[3].parse::<i64>().unwrap_or(0);
                blockchain.transfer(from, to, amount); // Queue a transaction
            }
            Some(&"balance") if args.len() == 2 => {
                let id = args[1].to_string();
                blockchain.get_balance(&id); // Display account balance
            }
            Some(&"exit") => break, // Exit the program
            _ => println!("Invalid command or arguments."), // Error for unrecognized input
        }
    }
}
