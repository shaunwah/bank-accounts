use chrono::{DateTime, Utc};
use rand::Rng;

#[derive(Debug)]
pub struct BankAccount {
    name: String,
    number: String,
    balance: f32,
    created_on: DateTime<Utc>,
    closed_on: Option<DateTime<Utc>>,
}

impl BankAccount {
    pub fn new(name: &str, balance: f32) -> Self {
        Self {
            name: String::from(name),
            number: Self::generate_number(),
            balance,
            created_on: Utc::now(),
            closed_on: None,
        }
    }

    fn generate_number() -> String {
        rand::thread_rng().gen_range(100_000_000..999_999_999).to_string()
    }

    pub fn get_balance(&self) -> f32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: f32) {
        if self.closed_on.is_none() {
            self.balance += amount;
            println!("Deposited ${} at {}. Current balance: ${}", amount, Utc::now(), self.balance);
        } else {
            println!("Failed to deposit ${}.", amount)
        }
    }

    pub fn withdraw(&mut self, amount: f32) {
        if self.closed_on.is_none() && amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            println!("Withdrawn ${} at {}. Current balance: ${}", amount, Utc::now(), self.balance);
        } else {
            println!("Failed to withdraw ${}.", amount)
        }
    }

    pub fn close(&mut self) {
        if self.closed_on.is_none() {
            let time_now= Utc::now();
            self.closed_on = Some(time_now);
            println!("Account closed at {}.", time_now);
        } else {
            println!("Account is already closed!");
        }
    }
}