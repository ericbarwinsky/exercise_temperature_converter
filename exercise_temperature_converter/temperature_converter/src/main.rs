use std::io;

/// The main entry point of the application.
/// It handles user interaction, reads input values, validates choices,
/// and displays the converted temperature result.
fn main() {
    println!("--- Temperature Converter ---");
    println!("Choose conversion direction:");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");

    // Read user choice from standard input
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    // Safely parse the user's choice into an unsigned 32-bit integer
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number (1 or 2).");
            return;
        }
    };

    // Validate that the choice is either 1 or 2
    if choice != 1 && choice != 2 {
        println!("Invalid choice. Please select 1 or 2.");
        return;
    }

    // Request the temperature value from the user
    println!("Enter the temperature value:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Safely parse the temperature input into a 64-bit floating-point number
    let temp: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for temperature.");
            return;
        }
    };

    // Call the appropriate conversion function based on the user's selection
    if choice == 1 {
        let result = celsius_to_fahrenheit(temp);
        println!("{:.1} °C is equivalent to {:.1} °F", temp, result);
    } else {
        let result = fahrenheit_to_celsius(temp);
        println!("{:.1} °F is equivalent to {:.1} °C", temp, result);
    }
}

/// Converts a temperature value from Celsius to Fahrenheit.
///
/// Formula: (C * 9.0 / 5.0) + 32.0
///
/// # Arguments
/// * `c` - The temperature value in degrees Celsius (f64)
///
/// # Returns
/// * The converted temperature value in degrees Fahrenheit (f64)
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

/// Converts a temperature value from Fahrenheit to Celsius.
///
/// Formula: (F - 32.0) * 5.0 / 9.0
///
/// # Arguments
/// * `f` - The temperature value in degrees Fahrenheit (f64)
///
/// # Returns
/// * The converted temperature value in degrees Celsius (f64)
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}