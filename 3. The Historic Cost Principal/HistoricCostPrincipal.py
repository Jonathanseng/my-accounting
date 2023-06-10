def historic_cost_principal_accounting(principal, interest_rate, number_of_periods):
  """
  This code will calculate the historic cost principal accounting for a loan with the specified principal amount, 
  interest rate, and number of periods. 
  The results will be printed to the console.
  Calculates the historic cost principal accounting for a loan.

  Args:
    principal: The principal amount of the loan.
    interest_rate: The interest rate of the loan.
    number_of_periods: The number of periods of the loan.

  Returns:
    A dictionary of the principal amounts for each period.
  """

  principal_amounts = {}
  for period in range(number_of_periods):
    interest = principal * interest_rate / 100
    principal_amount = principal - interest
    principal_amounts[period] = principal_amount

  return principal_amounts


if __name__ == "__main__":
  # Define the principal amount, interest rate, and number of periods.
  principal = 100_000
  interest_rate = 5
  number_of_periods = 10

  # Calculate the historic cost principal accounting.
  principal_amounts = historic_cost_principal_accounting(principal, interest_rate, number_of_periods)

  # Print the principal amounts for each period.
  for period, principal_amount in principal_amounts.items():
    print(f"Period {period}: Principal amount = {principal_amount}")
