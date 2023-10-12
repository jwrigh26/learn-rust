fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let mut my_vec: Vec<String> = Vec::new();
    // If we run the program now, the compiler will give an error.
    // It doesn't know the type of vec.

    my_vec.push(name1); // Now it knows: it's Vec<String>
    my_vec.push(name2);

    // Another easy wasy to create a vector is with the vec! macro
    let mut _my_vec2 = vec![String::from("Windy"), String::from("Gomesy")];
    let mut _my_vec3: Vec<u8> = vec![8, 10, 10];

    // Rember arrays are like
    let _array1 = [1, 2, 3, 4, 5];
    // the macro vec! is like an array just with vec! in front of the [].

    // We can declare all kinds of vecs. When declaring a vec we say what type it is.
    let _: Vec<i32> = Vec::new(); // We call it a "Vec of i32s".
    let _: Vec<String> = Vec::new(); // We call it a "Vec of Strings".

    // We can also slice a vector too, just like in an array.
    // For review array slices are like the following:
    let _array2 = [1, 2, 3, 4, 5];
    let _slice1 = &_array2[1..3]; // This is a slice of the array from index 1 to 3.

    // Remember that:
    // index numbers start at 0 (not 1)
    // index ranges are exclusive ( they do not include the last number)
    // So, this slice is from index 1 to 2 (not 3)
    // [0..2] menas teh first index and the second index ( 0 and 1 ). 
    // Or you can call it the "zeroth and the first" index. 
    // It doesn't have the third item, which is index 2.
    // You can also have an inclusive range, which means it includes
    // the last naumber too. To do this, add = to write ..= instead of ..
    // So instead of [0..2] you would write [0..=2]

    // For vectors it's like so
    let vect_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // This is a vector of 10 numbers.
    // Everything is the same as above except we added vec!.
    let three_to_five = &vect_of_ten[2..5];
    let start_at_two = &vect_of_ten[1..];
    let end_at_five = &vect_of_ten[..5];
    let everything = &vect_of_ten[..];

    println!("The vector is: {:?}", vect_of_ten);
    println!("The slice from index 2 to 5 is: {:?}", three_to_five);
    println!("The slice from index 1 to the end is: {:?}", start_at_two);
    println!("The slice from the start to index 5 is: {:?}", end_at_five);
    println!("The slice of the whole vector is: {:?}", everything);

    // Because a vec is slower than an array,
    // we can use some methods to make it faster.
    // A vec has a capcity, which means the space given to the vector.
    // When you push a new item on teh vector,
    // it get closer and closer to the capacity.
    // Then if you go past the capacity, it will make its capacity
    // double and copy the items into the new space.
    // Thi sis called reallocation. 
    // We'll use a method called .capcity() to look at the capacity 
    // of a vector as we add items to it.
    let mut my_vec = Vec::new();
    println!("The capacity of my_vec is: {}", my_vec.capacity());
    my_vec.push('a');
    println!("The capacity of my_vec is: {}", my_vec.capacity());
    my_vec.push('b');
    my_vec.push('c');
    my_vec.push('d');
    println!("The capacity of my_vec is: {}", my_vec.capacity());
    my_vec.push('e');
    println!("The capacity of my_vec is: {}", my_vec.capacity());
    // So this vector has two reallocations: 0 to 4, and 4 to 8. 
    // We can make it faster:
    // By do this:
    let _: Vec<char> = Vec::with_capacity(8);
    // This will make the capacity 8 from the start.
    // Which means this vector has 0 realloactions, which is better.
    // So if you think you know how many elements you need,
    // you can use Vec::with_capacity() to make it faster.

    // Last thing.
    // You remember that you can use :into() to make a &str into a String.
    // You can also use it to make an array into a Vec.
    // You have to tell .into() that you want a Vec, but you don't have to
    // choose the type of Vec. If you don't want to choose, you can write Vec<_>.
    let _: Vec<u8> = [1, 2, 3].into();
    let _: Vec<_> = [9, 0, 10].into();
    // Vec<_> means "choose the Vec type for me".
    // Rust will choose Vec<i32> because the array is i32s.

}
