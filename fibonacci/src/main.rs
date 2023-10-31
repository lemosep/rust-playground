use std::io;
fn main() {
    loop {
        println!("Type a positive number");

        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Error: couldn't read line");

        let value: u32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if value == 0 {
            break;
        }

        match value {
            1 => println!("The 1st value of fibonacci is {}", fib(value)),
            2 => println!("The 2nd value of fibonacci is {}", fib(value)),
            3 => println!("The 3rd value of fibonacci is {}", fib(value)),
            _ => println!("The {value}th value of fibonacci is {}", fib(value)),
        };
    }
}

//recursive fibonacci function
fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
