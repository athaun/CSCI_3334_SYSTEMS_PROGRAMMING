#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        return BankAccount {
            balance: initial_balance,
        };
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        return self.balance;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);

        let account = BankAccount::new(0.0);
        assert_eq!(account.balance(), 0.0);
    }

    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(10.0);
        account.deposit(5.0);
        assert_eq!(account.balance(), 15.0);

        account.deposit(0.0);
        assert_eq!(account.balance(), 15.0);

        account.deposit(-50.0);
        assert_eq!(account.balance(), 15.0);
    }

    #[test]
    fn test_deposit_with_precision() {
        let mut account = BankAccount::new(0.0);
        account.deposit(1.0 / 3.0);
        assert!((account.balance() - 0.3).abs() < 0.1);
    }

    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(10.0);
        account.withdraw(5.0);
        assert_eq!(account.balance(), 5.0);

        account.withdraw(0.0);
        assert_eq!(account.balance(), 5.0);

        account.withdraw(-50.0);
        assert_eq!(account.balance(), 5.0);

        account.withdraw(10.0);
        assert_eq!(account.balance(), 5.0);
    }

    #[test]
    fn test_withdraw_with_precision() {
        let mut account = BankAccount::new(1.0);
        account.withdraw(1.0 / 3.0);
        assert!((account.balance() - 0.7).abs() < 0.1);
    }

    #[test]
    fn test_balance() {
        let account = BankAccount::new(10.0);
        assert_eq!(account.balance(), 10.0);
    }
}