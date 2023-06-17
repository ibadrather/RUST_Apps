// 1:
// This will cause error as x is unmutable by default
// => cannot assign twice to immutable variable

// fn mutability1() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }



// 2, 3:

// constants -> mut not allowed
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// To change the value of x, we need to declare it mutable
fn mutability2() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Constant value is: {THREE_HOURS_IN_SECONDS}")
}

// 4: Shadowing
fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}


fn main() {
    // mutability1();

    mutability2();

    shadowing();
}
