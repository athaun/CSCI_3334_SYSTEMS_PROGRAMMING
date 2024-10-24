mod bank_account;

use bank_account::BankAccount;

fn main() {
    let mut account = BankAccount::new(100.0);
    println!("Initial balance: {}", account.balance());

    account.deposit(50.0);
    println!("Balance after deposit: {}", account.balance());

    account.withdraw(30.0);
    println!("Balance after withdrawal: {}", account.balance());

    account.withdraw(150.0);
    println!("Balance after attempting to withdraw more than balance: {}", account.balance());

    account.deposit(-20.0);
    println!("Balance after attempting to deposit a negative amount: {}", account.balance());

    account.withdraw(-20.0);
    println!("Balance after attempting to withdraw a negative amount: {}", account.balance());
}