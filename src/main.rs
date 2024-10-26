use std::io;

fn main() {
    loop {
        println!("-- Calculator app --");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 5 {
            println!("Exiting program!");
            break;
        }

        if choice > 5 || choice < 1 {
            println!("Wrong input!");
            continue;
        }

        println!("Enter first number: ");

        let mut first_number = String::new();

        io::stdin()
            .read_line(&mut first_number)
            .expect("Failed to read line");

        let first_number: f64 = match first_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        println!("Enter second number: ");

        let mut second_number = String::new();

        io::stdin()
            .read_line(&mut second_number)
            .expect("Failed to read line");

        let second_number: f64 = match second_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        match choice {
            1 => {
                let result = first_number + second_number;
                println!("Result: {}", result);
            }
            2 => {
                let result = first_number - second_number;
                println!("Result: {}", result);
            }
            3 => {
                let result = first_number * second_number;
                println!("Result: {}", result);
            }
            4 => {
                if second_number != 0.0 {
                    let result = first_number / second_number;
                    println!("Result: {}", result);
                } else {
                    println!("Error: Division by zero.");
                }
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 5.");
            }
        }
    }
}
