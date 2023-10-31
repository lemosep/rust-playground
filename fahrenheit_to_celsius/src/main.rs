use std::io;
fn main() {
    
    println!("What temperature unit you wish to convert to? (F or C)");
    let mut temp_unit: String = String::new();

    io::stdin()
        .read_line(&mut temp_unit)
        .expect("Error: failed to read line.");


    println!("Type your vaule");
    let mut value: String = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Error: failed to read line.");

   
    let value: f64 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => -1.00
    };

    match temp_unit.as_str() {
        "C\n" => println!("{}", fahrenheit_to_celsius(value)),
        "F\n" => println!("{}", celsius_to_fahrenheit(value)),
        _ => println!("Not an option")
    }

}

fn fahrenheit_to_celsius(val: f64) -> f64 {
    (val * (5.0 / 9.0)) + 32.0
}

fn celsius_to_fahrenheit(val: f64) -> f64 {
    val * (9.0 / 5.0) + 32.00
}

