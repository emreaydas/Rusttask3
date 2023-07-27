#![allow(dead_code)]

trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
        } else {
            println!("Insufficient balance. Withdrawal amount exceeds the account balance.");
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
     
    let mut account1 = BankAccount {
        account_number: 123456,
        holder_name: String::from("Ali Koç"),
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: 987654,
        holder_name: String::from("Aziz Yıldırım"),
        balance: 500.0,
    };

    account1.deposit(500.0);

    account2.withdraw(200.0);

    println!("Account 1 Balance: {}", account1.balance());
    println!("Account 2 Balance: {}", account2.balance());
}