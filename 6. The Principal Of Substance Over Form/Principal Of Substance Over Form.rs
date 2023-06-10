// we define a Transaction struct to represent a financial transaction. Each transaction has a description, an amount, and a transaction type (revenue, expense, asset, or liability). We also define a FinancialStatement struct to hold the transactions.

// The FinancialStatement struct has methods to add transactions and calculate the net income. The add_transaction method allows you to add transactions to the financial statement.

// In the main function, we create a FinancialStatement instance and add transactions to it. Finally, we calculate and print the net income based on the revenues and expenses.

// The key consideration for applying the principle of substance over form is to ensure that the transactions recorded accurately represent the economic reality of the business, even if the legal or contractual form may suggest otherwise. This involves careful analysis and interpretation of the transactions and their impact on the financial statements.

// Please note that the example provided is a simplified demonstration and does not cover more advanced accounting concepts or considerations. The focus is on capturing different transaction types and calculating net income, highlighting the need for proper interpretation of transactions in line with the principle of substance over form.

struct Transaction {
    description: String,
    amount: f64,
    transaction_type: TransactionType,
}

enum TransactionType {
    Revenue,
    Expense,
    Asset,
    Liability,
}

struct FinancialStatement {
    transactions: Vec<Transaction>,
}

impl FinancialStatement {
    fn new() -> FinancialStatement {
        FinancialStatement {
            transactions: Vec::new(),
        }
    }

    fn add_transaction(&mut self, description: &str, amount: f64, transaction_type: TransactionType) {
        let transaction = Transaction {
            description: String::from(description),
            amount,
            transaction_type,
        };
        self.transactions.push(transaction);
    }

    fn calculate_net_income(&self) -> f64 {
        let total_revenues: f64 = self
            .transactions
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Revenue)
            .map(|t| t.amount)
            .sum();
        let total_expenses: f64 = self
            .transactions
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Expense)
            .map(|t| t.amount)
            .sum();
        total_revenues - total_expenses
    }
}

fn main() {
    let mut financial_statement = FinancialStatement::new();

    financial_statement.add_transaction("Product Sales", 1000.0, TransactionType::Revenue);
    financial_statement.add_transaction("Cost of Goods Sold", 600.0, TransactionType::Expense);
    financial_statement.add_transaction("Depreciation Expense", 100.0, TransactionType::Expense);
    financial_statement.add_transaction("Equipment Purchase", 1500.0, TransactionType::Asset);

    let net_income = financial_statement.calculate_net_income();
    println!("Net Income: {:.2}", net_income);
}
