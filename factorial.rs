use std::io::{self, Write};

// demonstrates Recurstion. 


fn main() {
    let mut menu_choice = 0;
    while  menu_choice != 8{
        println!("1. Find factorial");
        println!("2. Find fibonacci");
        println!("3. ");
        println!("4. ");
        println!("5. ");
        println!("6. ");
        println!("7. Help");
        println!("8. Quit ");
        print!("Enter your choice: ");
        let mut raw_input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut raw_input).unwrap();
        menu_choice = raw_input.trim().parse().unwrap();

        if 1 == menu_choice{
            find_factorial();
        }
        else if 2 == menu_choice{ }
        else if 3 == menu_choice{ }
        else if 7 == menu_choice{
            print!("{}", info("factorial"));
        }
        else{}
    }
 

}

fn info(opperation: &str) -> String{

    let mut opperation_defs = Vec::new();

    opperation_defs.push("A factorial of a non-negative integer n is the product of all positive integers less than or equal to n. It is denoted by n! and is defined as:
    
    n! = n * (n-1) * (n-2) * ... * 1
    
    For example:
    5! = 5 * 4 * 3 * 2 * 1 = 120
    3! = 3 * 2 * 1 = 6
    0! = 1 (by definition)\n\n");

    
    let mut output = String::new();

    if "factorial" == opperation {
        
        output = opperation_defs.get(0).expect("REASON").to_string();
        
    }
    return output;
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