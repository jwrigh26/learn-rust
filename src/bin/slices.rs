// Slices are references to contiguous sequences of elements in a collection rather than the whole collection.
// Slices have a fixed size.
// Slices don't have ownership.
// Slices are immutable by default.
// -------------------------------------
// Let's learn about Slices
fn main() {
    // Slices let you refercne a contguous (sharing a common border; touching) sequence of elements.
    // In other words Image you have a line of toy blocks placed right next to each other, with no spaces in between.
    // Each block is touching the next one on either side. This line of blocks is like a "continous sequence of elements".
    // In simpler words, it's a bunch of things lined up together without any gaps in between!

    // Problem:
    // Write a function that takes a string of words separated by spaces
    // and retursn the first word it finds in that string.
    // If the function doesn't find a space in the string, 
    // the whole string must be one word, 
    // so the entire string should be returned.

    // What should we return?
    // &String is a parameter. This doesn't take ownerhsip and that is fine.
    // But what should we return? ( usize ??? )
    // fn first_word(s: &String) - ?

    // First attempt #1
    let mut s = String::from("hello world");
    let _word = first_word_a(&s); // word will get the value 5
    s.clear(); // this empties teh String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is not totally invalid!
    // ---
    // Having to worry about the index in 'word' getting out of sync with the data in 's'
    // is tedious and error prone!
    // This is why we have slices!!!
    // A string slice is a reference to part of a 'String',
    // and it loos like this:
    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];
    // Rather than a reference to the entire String, hello is a reference to a portion of the String,
    // specified in the exta [0..5] bit.
    // We create slices using a range within brackets by specifying
    // [staring_index..ending_index],
    // starting_index is the first position in the slice and 
    // ending_index is one more than the last position in the slice.
    // Internally the slice data structure stores the starting position
    // and the length of the slice, which corresponds to the ending_index
    // minus staring_index. So, in teh case of let world = &s[6..11];,
    // world would be a slice that contains a pointer to the byte at index 6
    // of s with a lenght of value 5.
    // 6 | 7 | 8 | 9 | 10 |
    // w | o | r | l |  d |
    // With Ruts's '..' range syntax, if you want to start at index 0, you can drop the value before the two periods.
    // In other words, these are equal:
    let s = String::from("hello");
    let _slice = &s[0..2];
    let _slice = &s[..2];
    // By the same token, you slice includes the last byte of the String, you can drop the trailing number. 
    // That means these are equal:
    let s = String::from("hello");

    let len = s.len();

    let _slice = &s[3..len];
    let _slice = &s[3..];

    // You can also drop both values to take a slice of the entier string. So these are equal:
    let s = String::from("hello");

    let len = s.len();

    let _slice = &s[0..len];
    let _slice = &s[..];

    // String Slices as Parameters
    // Knowing that you can take slices of literals and String values leads us to one more imporvement on first_word,
    // and that's it's signature:
    // Instead of 
    // fn first_word(s: &String) -> &str 
    // We can do this instead
    // fn first_word(s: &str) -> &str
    // If we have a string slice, we can pass that directly. If we have a String, we can pass a slice
    // of the String or a reference to the String.
    // This felxibility takes advantage of deref coercisons
    // ( https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods )
    // Defining a funtion to take a string slice instead of a refrence to a String makes
    // The function more general and useful without losing any functionality.

    // Other slices
    // Numbers
    let a = [1,2,3,4,5];
    let a_slice  = &a[1..3];
    assert_eq!(a_slice, &[2, 3]);
    // The slice has the type '&[i32]'. It works the same way as string slices do, by storing a reference
    // to the first element and a lenght. You'll use this kind of slice for all sorts of other collections.

}


// #1
// The problem with this function is we're returning a 'usize' on its own,
// but it's only a meaningful number in the context of the '&String'.
// In other words, because it's a spearate value form the String, 
// there's no guarantee that it will still be valid in the future. 
fn first_word_a(s: &String) -> usize {
    let bytes = s.as_bytes();  // unsigned meaning no negatives and starts at 0.
    // iter is a method that returns each element in a collection
    // enumerate wraps the result of iter and returns each element
    // as a part of a tuple isntead.
    // First element of the tuple returned from enumerate is the index,
    // and the second element is a reference to the element.
    for (i, &item) in bytes.iter().enumerate() {
        // Use the byte literal syntax to search 
        // for the byte that represents the space ' '
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


// #2
// With what we've learned about slices and ranges, let's try again!
// We get teh index for the end of the word the same way we did in the first attempt
// by looking for the first occurrence of a space.
// When we find a space, we return a string slice using the start of the string and the
// index of the space as the starting and ending indices.
// Now when we call first_word, we get backa single value that is tied to the underlying data.
// The value is made up of a reference to the starting point of the slice and the number of 
// elments in the slice.
// The safety in this function is the slice version will throw a compile-time error.
// Unlike the first example that wouldn't show an error unles you tried to use 'word"
// after clearing the 's' variable.
fn first_word_b(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
