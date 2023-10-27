// Enums, Checkout main for more info
#[allow(dead_code)]
enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

// We pass a refercne for Mood into this function
// It then imports all Moods using the use keyword
// and then matches the mood to a happiness level
// Retuns a i32 to represent the happiness level
fn match_mood(mood: &Mood) -> i32 {
    // Here we type Mood::Value every time
    // To bypass this we can use the use keyword
    use Mood::*;
    let happiness_level = match mood {
        Happy => 10, // We dont' have to write Mood::Happy anymore thanks to use keyword
        Sleepy => 6,
        NotBad => 7,
        Angry => 2,
    };
    happiness_level
}

// Parts of an enum can also be turned into an integer. That's because Rust gives each arm of an enum
// a number that starts with 0 for its own use. You can do things with it if your enum doesn't have any
// other data in it.
#[derive(Debug, Copy, Clone)]
enum Season {
    Spring, // If this was Spring(String) or something it wouldn't work
    Summer,
    Autumn,
    Winter,
}

// We derive the same but instead of relying on clone
// we reference the enum's vec down below
// and then to convert it to an u32 we deference it
#[derive(Debug, Copy, Clone)]
enum Stars {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowDwarf = 100,
    WhiteDwarf = 1000,
    RedGiant = 100_000,
    DeadStart, // This about this one. What number will it have?
}

// Enums to use multiple types
enum Number {
    U32(u32),
    I32(i32),
}

fn get_number(input: i32) -> Number {
    let number = match input.is_positive() {
        true => Number::U32(input as u32), // change it to use u32 if it's positive
        false => Number::I32(input), // otherwise just give the number because it's already i32.
    };
    number
}

// -------------------------------------
// Let's learn about Enums
fn main() {
    println!("Enums Getting Started");

    // Mood Example
    // Highglights the use of the match keyword and use keyword
    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&my_mood);
    println!("-------------------");
    println!(" ");
    println!("Out of 1 to 10, my happiness level is {}", happiness_level);
    // Seasons Example
    // uses the integer value of the enum
    // We print by deriving Debug, Copy, Clone
    println!(" ");
    println!("-------------------");
    println!(" ");
    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        let int_value = season as u32;
        println!("The season {:?}'s int value:  {}", season, int_value);
    }
    // Star example
    // uses assigned integers
    // We print by deferenecing the enum
    use Stars::*;
    let stars: Vec<Stars> = vec![
        BrownDwarf,
        RedDwarf,
        YellowDwarf,
        WhiteDwarf,
        RedGiant,
        DeadStart,
    ];
    for star in &stars {
        println!("The star {:?}'s int value:  {}", star, *star as u32);
    }
    // Enums to use multiple types
    println!(" ");
    println!("-------------------");
    println!(" ");
    // You know that items in a Vec, array, etc. all need the same type (only tuples are different).
    // But you can actually use an enum to put different types in.
    // Imageine we want to have a Vec with u32s or i32s. Of course you can make a Vec<(u32, i32)>,
    // Which is a vec with (u32, i32) tupes, but
    // we only want one each time. So here you can use an enum.
    let my_numbers = vec![get_number(-800), get_number(8)];

    for item in my_numbers {
        match item {
            Number::U32(number) => println!("This is a u32: {}", number),
            Number::I32(number) => println!("This is a i32: {}", number),
        }
    }
}
