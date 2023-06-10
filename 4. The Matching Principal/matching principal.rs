// we define a Transaction struct to represent a financial transaction. Each transaction has a description, an amount, and a transaction type (revenue or expense). We also define a FinancialStatement struct to hold the revenues and expenses.

//The FinancialStatement struct has methods to add revenues and expenses to their respective vectors. The calculate_net_income method calculates the net income by summing up the amounts of all revenues and expenses.

//In the main function, we create a FinancialStatement instance and add revenues and expenses to it. Finally, we calculate and print the net income.

//Note that this code is a simplified example and does not cover more advanced accounting concepts or considerations, such as accruals, depreciation, or adjustments. It is intended to demonstrate the basic structure of recording revenues and expenses for the purpose of understanding the matching principle.

struct Transaction {
    description: String,
    amount: f64,
    transaction_type: TransactionType,
}

enum TransactionType {
    Revenue,
    Expense,
}

struct FinancialStatement {
    revenues: Vec<Transaction>,
    expenses: Vec<Transaction>,
}

impl FinancialStatement {
    fn new() -> FinancialStatement {
        FinancialStatement {
            revenues: Vec::new(),
            expenses: Vec::new(),
        }
    }

    fn add_revenue(&mut self, description: &str, amount: f64) {
        let transaction = Transaction {
            description: String::from(description),
            amount,
            transaction_type: TransactionType::Revenue,
        };
        self.revenues.push(transaction);
    }

    fn add_expense(&mut self, description: &str, amount: f64) {
        let transaction = Transaction {
            description: String::from(description),
            amount,
            transaction_type: TransactionType::Expense,
        };
        self.expenses.push(transaction);
    }

    fn calculate_net_income(&self) -> f64 {
        let total_revenues: f64 = self.revenues.iter().map(|r| r.amount).sum();
        let total_expenses: f64 = self.expenses.iter().map(|e| e.amount).sum();
        total_revenues - total_expenses
    }
}

fn main() {
    let mut financial_statement = FinancialStatement::new();

    financial_statement.add_revenue("Product Sales", 1000.0);
    financial_statement.add_expense("Advertising", 200.0);
    financial_statement.add_expense("Rent", 300.0);
    financial_statement.add_revenue("Consulting Fees", 500.0);

    let net_income = financial_statement.calculate_net_income();
    println!("Net Income: {:.2}", net_income);
}
