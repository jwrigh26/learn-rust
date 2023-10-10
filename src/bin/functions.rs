use colored::*;

fn main () {
    println!("{}", "FUNCTIONS".blue().bold());
    another_function();
    another_function_with_parameter(5);
    let x = another_function_with_parameter_and_return_value(5);
    println!("The value of x, which is returned is: {} because the fn added + 1", format!("{}", x).yellow());
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_parameter(x: i32) {
    // statements are instructions that perform some action and do not return a value.
    println!("The value of x is: {}", format!("{}", x).green());
}

fn another_function_with_parameter_and_return_value(x: i32) -> i32 {
    // expressions evaluate to a resulting value.
    x + 1
}