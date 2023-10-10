use colored::*;

fn main() {
    // VARIABLES
    // --------------------
    println!("{}", "VARIABLES".blue().bold());
    // make it mutable so we can change it
    let mut x = 5;
    println!("The value of x is: {}", format!("{}", x).green());
    // x = 6; // error[E0384]: cannot assign twice to immutable variable `x`
    x = 6;
    println!("The value of x is: {}", format!("{}", x).yellow());

    // CONSTANTS
    // --------------------
    println!("{}", "CONSTANTS".blue().bold());
    // Like immutable varibles, constants are values that are bound to a name and are not allowed to change,
    // but there are a few differences between constants and variables.
    // First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—
    // they’re always immutable.
    // Second, you aren’t allowed to assign a value to a constant directly in your code.
    // Instead, you must use a constant expression, which is an expression whose value can be computed at compile time.
    // Constants can be declared in any scope, including the global scope,
    // which makes them useful for values that many parts of code need to know about.
    const MAX_POINTS: u32 = 100_000;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "The value of MAX_POINTS is: {}",
        format!("{}", MAX_POINTS).green()
    );
    println!(
        "The value of THREE_HOURS_IN_SECONDS is: {}",
        format!("{}", THREE_HOURS_IN_SECONDS).yellow()
    );

    // SHADOWING
    // --------------------
    println!("{}", "SHADOWING".blue().bold());
    // Shadowing is different from marking a variable as mut,
    //because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword.

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!(
            "The value of x in the inner scope is: {}",
            format!("{}", x).green()
        );
    }
    println!("The value of x is: {}", format!("{}", x).yellow());

    // SCALAR TYPES
    // --------------------

    // Print the min and max of a i8
    println!("{}", "SCALAR TYPES".blue().bold());
    println!("{}", "i8".green());
    println!("Min: {}", i8::min_value());
    println!("Max: {}", i8::max_value());

    // Print the min and max of a u8
    println!("{}", "u8".green());
    println!("Min: {}", u8::min_value());
    println!("Max: {}", u8::max_value());

    // print the difference betwee i8 and u8
    println!("{}", "i8 - u8".green());
    println!("The difference between i8 and u8 is \nunsigned variants can store numbers from 0 to 255, \nwhile signed variants can store numbers from -128 to 127");

    println!(
        "{}",
        "FLOATING-POINT TYPES AND NUMBERIC OPERATIONS".blue().bold()
    );

    // addition
    let sum = 5 + 10;
    println!(
        "The value of sum (5 + 10) is: {}",
        format!("{}", sum).green()
    );

    // subtraction
    let difference = 95.5 - 4.3;
    println!(
        "The value of difference (95.5 - 4.3) is: {}",
        format!("{}", difference).yellow()
    );

    // multiplication
    let product = 4 * 30;
    println!(
        "The value of product (4 * 30) is: {}",
        format!("{}", product).green()
    );

    // division
    let quotient = 56.7 / 32.2;

    // Here is a bit more breakdown of the division operator.
    // 1. '-5' is the dividend and is divided by 3, the divisor.
    //    this equals -1.6666666666666667
    // 2. The integer  part is '-1' and the fractional part is '0.6666666666666667'
    // 3. The fractional part is truncated and the result is -1
    //    Meaning teh '/' operator truncates the result towards zero, so you are left with -1.
    //    This behavior ensure that the division operation is fast, as it doesn't need to round
    //    or take the floor of the result, but it can be surprising if your expecting a different
    //    rounding behavior.
    let truncated = -5 / 3; // Results in -1
    println!(
        "The value of quotient (56.7 / 32.2) is: {}",
        format!("{}", quotient).yellow()
    );
    println!(
        "The value of truncated (-5 / 3) is: {}",
        format!("{}", truncated).yellow()
    );

    // remainder
    let remainder = 43 % 5;
    println!(
        "The value of remainder (43 % 5) is: {}",
        format!("{}", remainder).green()
    );

    
    // BOOLEAN TYPES
    // --------------------
    let _t = true;
    let _f: bool = false; // with explicit type annotation
    
    // CHARACTER TYPES
    // --------------------
    let _c = 'z';
    let _z: char = 'Z'; // with explicit type annotation
    let _hearted_eyed_cat = '😻';
    // Note that char literals are specified with single quotes, as opposed to string literals,
    // which use double quotes. Rust's char type is four bytes in size and represents a Unicode Scalar Value...
    println!("{}", "!!! Skipping booleans and chars !!!".yellow().bold());

    // COMPOUND TYPES
    // --------------------
     println!("{}", "COMPOUND TYPES".blue().bold());
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup; // destructuring
    println!(
        "The value of x, y, z is: {}, {}, {}",
        format!("{}", x).green(),
        format!("{}", y).yellow(),
        format!("{}", z).blue()
    );

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundered = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!(
        "The value of five_hundered, six_point_four, one is: {}, {}, {}",
        format!("{}", five_hundered).green(),
        format!("{}", six_point_four).yellow(),
        format!("{}", one).blue()
    );
}
