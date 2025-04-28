trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Deposit amount must be greater than zero.".to_string())
        } else {
            self.balance += amount;
            Ok(())
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Withdrawal amount must be greater than zero.".to_string())
        } else if amount > self.balance {
            Err("Insufficient funds.".to_string())
        } else {
            self.balance -= amount;
            Ok(())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount {
        account_number: 1001,
        holder_name: String::from("Alice Smith"),
        balance: 500.0,
    };

    let mut account2 = BankAccount {
        account_number: 1002,
        holder_name: String::from("Bob Johnson"),
        balance: 1000.0,
    };

    match account1.deposit(200.0) {
        Ok(()) => println!("Deposit successful on account 1."),
        Err(e) => println!("Error depositing to account 1: {}", e),
    }

    match account2.withdraw(150.0) {
        Ok(()) => println!("Withdrawal successful on account 2."),
        Err(e) => println!("Error withdrawing from account 2: {}", e),
    }

    println!("Account 1 balance: ${}", account1.balance());
    println!("Account 2 balance: ${}", account2.balance());
}
