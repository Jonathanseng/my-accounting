// In this example, we define a Product struct with fields for name, quantity, and cost. The new function is used to create instances of Product. The calculate_cost method calculates the total cost of a product based on its quantity and cost.

// In the main function, we create a vector of Product instances and iterate over them to calculate the total cost. The total_cost variable keeps track of the cumulative cost. Finally, we print the total cost to the console.

// Please note that this is a simplified example and doesn't include more advanced accounting concepts or calculations. It's intended to demonstrate the basic structure of the code for cost principal accounting.

struct Product {
    name: String,
    quantity: u32,
    cost: f64,
}

impl Product {
    fn new(name: &str, quantity: u32, cost: f64) -> Product {
        Product {
            name: String::from(name),
            quantity,
            cost,
        }
    }

    fn calculate_cost(&self) -> f64 {
        self.quantity as f64 * self.cost
    }
}

fn main() {
    let products = vec![
        Product::new("Product 1", 10, 5.0),
        Product::new("Product 2", 5, 7.5),
        Product::new("Product 3", 3, 12.0),
    ];

    let mut total_cost = 0.0;
    for product in &products {
        total_cost += product.calculate_cost();
    }

    println!("Total cost: {:.2}", total_cost);
}
