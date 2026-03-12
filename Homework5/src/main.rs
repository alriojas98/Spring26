#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount >= 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount >= 0.0 && amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Insufficient funds. Withdrawal failed.");
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert_eq!(account.balance(), 150.0);
    }

    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(40.0);
        assert_eq!(account.balance(), 60.0);
    }

    // Add more tests here
    #[test]
    fn test_deposit_negative() {
        let mut account = BankAccount::new(100.0);
        account.deposit(-50.0);
        assert_eq!(account.balance(), 100.0); // should be unchanged
    }

    #[test]
    fn test_withdraw_more_than_balance() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(200.0);
        assert_eq!(account.balance(), 100.0); // should be unchanged
    }

    #[test]
    fn test_withdraw_exact_balance() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(100.0);
        assert_eq!(account.balance(), 0.0);
    }
}

fn main() {
    println!("=== BankAccount Demo ===");

    let mut account = BankAccount::new(1000.0);
    println!("Initial balance: ${:.2}", account.balance());

    account.deposit(500.0);
    println!("After depositing $500.00: ${:.2}", account.balance());

    account.withdraw(200.0);
    println!("After withdrawing $200.00: ${:.2}", account.balance());

    account.deposit(-100.0); // ignored
    println!("After invalid deposit of -$100.00: ${:.2}", account.balance());

    account.withdraw(9999.0); // insufficient funds
    println!("After failed withdrawal of $9999.00: ${:.2}", account.balance());
}