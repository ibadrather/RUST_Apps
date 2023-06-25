// Ownership Rules
// First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

// 1. Each value in Rust has an owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.


fn main() {

    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
        println!("s inside a scope: {s}");
    }                      // this scope is now over, and s is no longer valid

    // String

    let s = String::from("Hello string literal");

    println!("{s}");

    // This kind of string can be mutated:

    let mut s = String::from("Hello");
    s.push_str(", world! (using push_str on a mutable string)"); // push_str() appends a literal to a String

    println!("{s}");

    {
        let mut s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
        s.push_str(", world! (using push_str on a mutable string inside a scope)")
    }                                  // this scope is now over, and s is no
                                       // longer valid
    
    
    // THis will throw an error
    // let s1 = String::from("hello");
    // let s2 = s1;
    
    let s1 = String::from("hello s1");
    let s2 = s1.clone();

    println!("{}, world!", s1);
    println!("{}, world!", s2);

}

