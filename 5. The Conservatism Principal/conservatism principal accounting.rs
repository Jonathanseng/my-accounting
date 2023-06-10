struct FinancialStatement {
    assets: f64,
    liabilities: f64,
}

impl FinancialStatement {
    fn new(assets: f64, liabilities: f64) -> FinancialStatement {
        FinancialStatement {
            assets,
            liabilities,
        }
    }

    fn calculate_equity(&self) -> f64 {
        self.assets - self.liabilities
    }

    fn calculate_conservative_equity(&self) -> f64 {
        let estimated_assets = self.assets * 0.9; // Conservatively estimate assets as 90% of their actual value
        let estimated_liabilities = self.liabilities * 1.1; // Conservatively estimate liabilities as 110% of their actual value
        estimated_assets - estimated_liabilities
    }
}

fn main() {
    let financial_statement = FinancialStatement::new(100000.0, 50000.0);
    let equity = financial_statement.calculate_equity();
    let conservative_equity = financial_statement.calculate_conservative_equity();

    println!("Equity: {:.2}", equity);
    println!("Conservative Equity: {:.2}", conservative_equity);
}
