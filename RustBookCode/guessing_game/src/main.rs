use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut num_of_guesses = 0;

    loop {
        let mut guess = String::new(); // mutable variable

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim()
        //                 .parse()
        //                 .expect("Please type a number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        
        // Compare guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
        }
        
        num_of_guesses += 1;

        println!("Number of guesses taken: {num_of_guesses}")

    }
    println!("Number of guesses taken: {num_of_guesses}")
}