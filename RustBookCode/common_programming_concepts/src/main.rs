
// Main function that we use to run everything
fn main() {
    println!("This is the main function");

    another_function();

    five();

    what_is_the_number(20);

    print_labeled_measurement(5, 'm');

    function_with_expression();

    let num: i32 = 19;
    let res = plus_one(num);
    println!("{num} + 1 = {res}");
}

// Simple function that prints somethinf
fn another_function() {
    println!("This is another function");
}

fn five() -> i32 {
    5
}

fn what_is_the_number(x: i32) {
    println!("The number is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


fn function_with_expression() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}