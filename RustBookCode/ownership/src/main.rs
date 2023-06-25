// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. 
// A slice is a kind of reference, so it does not have ownership.

// Here’s a small programming problem: write a function that takes a string of words separated by spaces and 
// returns the first word it finds in that string. If the function doesn’t find a space in the string, 
// the whole string must be one word, so the entire string should be returned.

// Without using Slices
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_using_slices(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    println!("{word}");

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!


    // # String Slices:
    // let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    let s = String::from("hello world");


    let word_slices = first_word_using_slices(&s);
    println!("{word_slices}");
}