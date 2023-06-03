mod bank_account;

use bank_account::BankAccount;

fn main() {
    let mut account = BankAccount::new("John", 1000.0);
    println!("{:?}", account);
    account.deposit(100.0);
    account.deposit(200.0);
    account.withdraw(300.0);
    account.deposit(300.0);
    account.deposit(400.0);
    account.withdraw(500.0);
    account.deposit(500.0);
    println!("The current balance is {}.", account.get_balance());
    println!("{:?}", account);
    account.close();
    account.withdraw(350.0);
    println!("{:?}", account);
}
