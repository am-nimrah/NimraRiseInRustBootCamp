use std::io;
// Define the enum Operation to represent different arithmetic operations
enum Operation {
    Add(f64, f64),       // Variant Add holds a tuple of two f64 values
    Subtract(f64, f64),  // Variant Subtract holds a tuple of two f64 values
    Multiply(f64, f64),  // Variant Multiply holds a tuple of two f64 values
    Divide(f64, f64),    // Variant Divide holds a tuple of two f64 values
}
// Define the calculate function that performs arithmetic operations based on the Operation enum variant
fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(x, y) => x + y,            // Perform addition
        Operation::Subtract(x, y) => x - y,       // Perform subtraction
        Operation::Multiply(x, y) => x * y,       // Perform multiplication
        Operation::Divide(x, y) => {
            if y != 0.0 {
                x / y                            // Perform division, ensuring y is not zero
            } else {
                panic!("Division by zero!");     // Panic if division by zero is attempted
            }
        }
    }
}
fn main() {
    println!("🌟 Welcome to the Nimra's Rusty Math Sorcery! 🌟");
    println!("🧙‍♂️ Step into the realm of magical arithmetic with Merlin, the Nimra's Rusty Math Sorcery. 🧙‍♂️");
    println!("✨ Prepare to embark on an enchanting journey of numbers and spells! ✨\n");
    
    println!("Enter the first number:");            // Prompt user for input
    let mut input = String::new();                  // Create a mutable string to store user input
    io::stdin().read_line(&mut input).expect("Failed to read input"); // Read user input
    let num1: f64 = input.trim().parse().expect("Invalid number");     // Parse input into f64
    println!("Enter the operation (+, -, *, /):");  // Prompt user for operation
    input.clear();                                   // Clear previous input
    io::stdin().read_line(&mut input).expect("Failed to read input"); // Read user input
    let operation = match input.trim() {
        "+" => Operation::Add(num1, {
            println!("Enter the second number:");           // Prompt user for input
            input.clear();                                   // Clear previous input
            io::stdin().read_line(&mut input).expect("Failed to read input"); // Read user input
            let num2: f64 = input.trim().parse().expect("Invalid number");     // Parse input into f64
            num2
        }),          // Variant Add with tuple of num1 and num2
        "-" => Operation::Subtract(num1, {
            println!("Enter the second number:");           // Prompt user for input
            input.clear();                                   // Clear previous input
            io::stdin().read_line(&mut input).expect("Failed to read input"); // Read user input
            let num2: f64 = input.trim().parse().expect("Invalid number");     // Parse input into f64
            num2
        }),     // Variant Subtract with tuple of num1 and num2
        "*" => Operation::Multiply(num1, {
            println!("Enter the second number:");           // Prompt user for input
            input.clear();                                   // Clear previous input
            io::stdin().read_line(&mut input).expect("Failed to read input"); // Read user input
            let num2: f64 = input.trim().parse().expect("Invalid number");     // Parse input into f64
            num2
        }),     // Variant Multiply with tuple of num1 and num2
        "/" => Operation::Divide(num1, {
            println!("Enter the second number:");           // Prompt user for input
            input.clear();                                   // Clear previous input
            io::stdin().read_line(&mut input).expect("Failed to read input"); // Read user input
            let num2: f64 = input.trim().parse().expect("Invalid number");     // Parse input into f64
            num2
        }),       // Variant Divide with tuple of num1 and num2
        _ => panic!("Invalid operation"),            // Panic if invalid operation entered
    };
    // Create an Operation enum instance with parsed input values
    let result = calculate(operation); // Call calculate function with Operation enum instance
    println!("Result: {}", result);              // Print result to console
}
