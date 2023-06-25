// References and Borrowing

fn main() {

    // References
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Mutable Reference
    let mut s = String::from("hello");

    println!("Current value of s: {s}");

    change(&mut s);

    println!("The new value of s: {s}");

}

// Reference 
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// Mutable Reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
