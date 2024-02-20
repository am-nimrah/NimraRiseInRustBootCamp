use std::io;

// Define a struct to represent a product
#[derive(Debug)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

impl Product {
    // Method to create a new product
    fn new(name: String, description: String, price: f64, quantity: u32) -> Self {
        Product {
            name,
            description,
            price,
            quantity,
        }
    }
}

// Define a struct to represent the inventory
struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    // Method to create a new empty inventory
    fn new() -> Self {
        Inventory {
            products: Vec::new(),
        }
    }

    // Method to add a product to the inventory
    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    // Method to remove a product from the inventory
    fn remove_product(&mut self, index: usize) {
        if index < self.products.len() {
            self.products.remove(index);
            println!("Product removed successfully.");
        } else {
            println!("Index out of range.");
        }
    }

    // Method to update product details
    fn update_product(&mut self, index: usize, name: String, description: String, price: f64, quantity: u32) {
        if let Some(product) = self.products.get_mut(index) {
            product.name = name;
            product.description = description;
            product.price = price;
            product.quantity = quantity;
            println!("Product updated successfully.");
        } else {
            println!("Index out of range.");
        }
    }

    // Method to generate a report of the inventory
    fn generate_report(&self) {
        println!("Inventory Report:");
        for (index, product) in self.products.iter().enumerate() {
            println!("{}. Name: {}, Description: {}, Price: ${:.2}, Quantity: {}", index + 1, product.name, product.description, product.price, product.quantity);
        }
    }
}

fn main() {
    let mut inventory = Inventory::new();

    println!("Welcome to Rusty Store Inventory Management System!");

    loop {
        println!("\nMenu:");
        println!("1. Add Product");
        println!("2. Remove Product");
        println!("3. Update Product");
        println!("4. Generate Report");
        println!("5. Exit");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => {
                println!("Enter product details:");
                println!("Name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name)
                    .expect("Failed to read line");

                println!("Description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description)
                    .expect("Failed to read line");

                println!("Price:");
                let mut price = String::new();
                io::stdin().read_line(&mut price)
                    .expect("Failed to read line");
                let price: f64 = match price.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid price format. Please enter a valid number.");
                        continue;
                    }
                };

                println!("Quantity:");
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity)
                    .expect("Failed to read line");
                let quantity: u32 = match quantity.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid quantity format. Please enter a valid number.");
                        continue;
                    }
                };

                inventory.add_product(Product::new(name.trim().to_string(), description.trim().to_string(), price, quantity));
            },
            Ok(2) => {
                println!("Enter the index of the product to remove:");
                let mut index = String::new();
                io::stdin().read_line(&mut index)
                    .expect("Failed to read line");
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid index format. Please enter a valid number.");
                        continue;
                    }
                };

                inventory.remove_product(index - 1);
            },
            Ok(3) => {
                println!("Enter the index of the product to update:");
                let mut index = String::new();
                io::stdin().read_line(&mut index)
                    .expect("Failed to read line");
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid index format. Please enter a valid number.");
                        continue;
                    }
                };

                println!("Enter updated product details:");
                println!("Name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name)
                    .expect("Failed to read line");

                println!("Description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description)
                    .expect("Failed to read line");

                println!("Price:");
                let mut price = String::new();
                io::stdin().read_line(&mut price)
                    .expect("Failed to read line");
                let price: f64 = match price.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid price format. Please enter a valid number.");
                        continue;
                    }
                };

                println!("Quantity:");
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity)
                    .expect("Failed to read line");
                let quantity: u32 = match quantity.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid quantity format. Please enter a valid number.");
                        continue;
                    }
                };

                inventory.update_product(index - 1, name.trim().to_string(), description.trim().to_string(), price, quantity);
            },
            Ok(4) => {
                inventory.generate_report();
            },
            Ok(5) => {
                println!("Generating final report before exiting:");
                inventory.generate_report();
                println!("Thank you for using Rusty Store Inventory Management System!");
                break;
            },
            _ => println!("Invalid choice. Please enter a number between 1 and 5."),
        }
    }
}
