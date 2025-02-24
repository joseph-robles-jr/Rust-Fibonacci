use std::io::{self, Write};

// demonstrates Recurstion. 


fn main() {
    let mut menu_choice = 0;
    while  menu_choice != 3{
        println!("1. Find factorial");
        println!("2. Find fibonacci");
        println!("3. Quit");
        print!("Enter your choice: ");
        let mut raw_input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut raw_input).unwrap();
        menu_choice = raw_input.trim().parse().unwrap();

        if 1 == menu_choice{
            find_factorial();
        }
        else{}
    }
 

}

fn find_factorial() {
    print!("Enter a whole number: ");
    io::stdout().flush().unwrap(); // flush the buffer to display the prompt
    let mut input = String::new(); //creates a new string
    io::stdin().read_line(&mut input).unwrap(); //reads the input and unwraps it
    let input:i64 = input.trim().parse().unwrap(); //parses the input and unwraps it
    let output:i64; //creates a new string
    output = factorial(input);
    println!("The factorial of {} is {}", input, output);

}

fn factorial(n: i64) ->i64{
    if n == 0 || n == 1 {
        return 1;
    }
    else {
        return n * factorial(n - 1);
    }
}