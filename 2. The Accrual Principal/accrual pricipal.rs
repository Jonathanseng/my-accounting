// This code will calculate the accrual principal accounting for a loan with the specified principal amount, interest rate, and number of periods. The results will be printed to the console.

use std::collections::VecDeque;

fn accrual_principal_accounting(principal: f64, interest_rate: f64, number_of_periods: u32) -> VecDeque<f64> {
  // Create a vector to store the principal amounts for each period.
  let mut principal_amounts = VecDeque::new();

  // Iterate over the number of periods.
  for _period in 0..number_of_periods {
    // Calculate the interest for the current period.
    let interest = principal * interest_rate / 100.0;

    // Calculate the principal amount for the current period.
    let principal_amount = principal - interest;

    // Add the principal amount to the vector.
    principal_amounts.push_back(principal_amount);
  }

  // Return the vector.
  principal_amounts
}

fn main() {
  // Define the principal amount, interest rate, and number of periods.
  let principal = 100_000.0;
  let interest_rate = 5.0;
  let number_of_periods = 10;

  // Calculate the accrual principal accounting.
  let principal_amounts = accrual_principal_accounting(principal, interest_rate, number_of_periods);

  // Print the principal amounts for each period.
  for principal_amount in principal_amounts {
    println!("{}", principal_amount);
  }
}
