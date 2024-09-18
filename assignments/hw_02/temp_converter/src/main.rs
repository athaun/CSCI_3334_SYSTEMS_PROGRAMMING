const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_POINT
}

fn main() {
    let mut temp: f64 = 32.0;
    temp = fahrenheit_to_celsius(temp);
    println!("{:.2}°F is {:.2}°C", 32.0, temp);
    temp = celsius_to_fahrenheit(temp);
    println!("{:.2}°C is {:.2}°F", 0.0, temp);

    for temp_f in 1..=5 {
        let temp_c = fahrenheit_to_celsius(temp_f as f64);
        println!("{:.2}°F is {:.2}°C", temp_f as f64, temp_c);
    }
}
