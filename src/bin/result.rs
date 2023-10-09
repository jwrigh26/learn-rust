fn main() {
    let is_ok = false;

    let result: Result<i32, &str> = if is_ok {
        Ok(42)
    } else {
        Err("An error occurred")
    };

    let new_result = result.map_err(|_| "A custome error message");

    match new_result {
        Ok(value) => println!("Result is {}", value),
        Err(error) => println!("Error: {}", error),
    }
}
