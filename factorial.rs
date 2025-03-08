use std::io::{self, Write};

// demonstrates Recurstion.

fn main() {
    let mut menu_choice = 0;
    while menu_choice != 10 {
        println!("1. Find factorial");
        println!("2. Find fibonacci");
        println!("3. Find the average");
        println!("4. Check if a number is prime");
        println!("5. Find the GCD of two numbers");
        println!("6. Find the LCM of two numbers");
        println!("7. Calculate the power of a number");
        println!("8. Calculate the sum of the first n natural numbers");
        println!("9. Help");
        println!("10. Quit");
        print!("Enter your choice: ");
        let mut raw_input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut raw_input).unwrap();
        menu_choice = raw_input.trim().parse().unwrap();

        if 1 == menu_choice {
            find_factorial();
        } else if 2 == menu_choice {
            find_fibonacci();
        } else if 3 == menu_choice {
            find_average();
        } else if 4 == menu_choice {
            find_prime();
        } else if 5 == menu_choice {
            find_gcd();
        } else if 6 == menu_choice {
            find_lcm();
        } else if 7 == menu_choice {
            find_power();
        } else if 8 == menu_choice {
            find_sum_of_natural_numbers();
        } else if 9 == menu_choice {
            println!("Help: This program demonstrates various mathematical functions.");
            println!("1. Find factorial: Calculates the factorial of a given number.");
            println!("2. Find fibonacci: Calculates the nth Fibonacci number.");
            println!("3. Find the average: Calculates the average of a series of numbers.");
            println!("4. Check if a number is prime: Checks if a given number is prime.");
            println!("5. Find the GCD of two numbers: Calculates the greatest common divisor of two numbers.");
            println!("6. Find the LCM of two numbers: Calculates the least common multiple of two numbers.");
            println!("7. Calculate the power of a number: Calculates the power of a base number raised to an exponent.");
            println!("8. Calculate the sum of the first n natural numbers: Calculates the sum of the first n natural numbers.");
        } else {}
    }
}

fn get_int() -> i64 {
    print!("Enter a whole number: ");
    io::stdout().flush().unwrap(); // flush the buffer to display the prompt
    let mut input = String::new(); // creates a new string
    io::stdin().read_line(&mut input).unwrap(); // reads the input and unwraps it
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid whole number.");
            get_int() // recursively call get_int again for valid input
        }
    }
}

fn find_factorial() {
    let input: i64 = get_int();
    let output: i64; // creates a new string
    output = factorial(input);
    println!("The factorial of {} is {}", input, output);
}

fn factorial(n: i64) -> i64 {
    if n == 0 || n == 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

fn find_fibonacci() {
    let input: i64 = get_int();
    let output: i64 = fibonacci(input);
    println!("The fibonacci of {} is {}", input, output);
}

fn fibonacci(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        return b;
    }
}

fn find_average() {
    println!("Enter the numbers to calculate the average. Enter a negative number to finish.");
    let mut numbers = Vec::new();

    loop {
        let number = get_int();
        if number < 0 {
            break;
        }
        numbers.push(number);
    }

    if numbers.is_empty() {
        println!("No numbers were entered.");
    } else {
        let sum: i64 = numbers.iter().sum();
        let average = sum as f64 / numbers.len() as f64;
        println!("The average of the entered numbers is {}", average);
    }
}

fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn find_prime() {
    let input: i64 = get_int();
    if is_prime(input) {
        println!("{} is a prime number.", input);
    } else {
        println!("{} is not a prime number.", input);
    }
}

fn find_gcd() {
    println!("Enter the first number:");
    let a: i64 = get_int();
    println!("Enter the second number:");
    let b: i64 = get_int();
    let result = gcd(a, b);
    println!("The GCD of {} and {} is {}", a, b, result);
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

fn find_lcm() {
    println!("Enter the first number:");
    let a: i64 = get_int();
    println!("Enter the second number:");
    let b: i64 = get_int();
    let result = lcm(a, b);
    println!("The LCM of {} and {} is {}", a, b, result);
}

fn find_power() {
    println!("Enter the base number:");
    let base: i64 = get_int();
    println!("Enter the exponent:");
    let exponent: i64 = get_int();
    let result = power(base, exponent);
    println!("{} raised to the power of {} is {}", base, exponent, result);
}

fn power(base: i64, exponent: i64) -> i64 {
    if exponent == 0 {
        return 1;
    }
    base * power(base, exponent - 1)
}

fn find_sum_of_natural_numbers() {
    let input: i64 = get_int();
    let result = sum_of_natural_numbers(input);
    println!("The sum of the first {} natural numbers is {}", input, result);
}

fn sum_of_natural_numbers(n: i64) -> i64 {
    (n * (n + 1)) / 2
}


