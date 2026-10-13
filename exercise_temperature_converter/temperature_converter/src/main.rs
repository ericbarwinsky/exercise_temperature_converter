fn main() {
    println!("--- Temperature Converter ---");

    let celsius: f64 = 25.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);

    println!("{:.1} °C is equivalent to {:.1} °F", celsius, fahrenheit);
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}