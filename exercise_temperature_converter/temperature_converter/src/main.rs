use std::io;

fn main() {
    println!("--- Temperature Converter ---");
    println!("Choose conversion direction:");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");

    // Read choice
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number (1 or 2).");
            return;
        }
    };

    if choice != 1 && choice != 2 {
        println!("Invalid choice. Please select 1 or 2.");
        return;
    }

    // Read temperature value
    println!("Enter the temperature value:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let temp: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for temperature.");
            return;
        }
    };

    // Calculate and display result
    if choice == 1 {
        let result = celsius_to_fahrenheit(temp);
        println!("{:.1} °C is equivalent to {:.1} °F", temp, result);
    } else {
        let result = fahrenheit_to_celsius(temp);
        println!("{:.1} °F is equivalent to {:.1} °C", temp, result);
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}