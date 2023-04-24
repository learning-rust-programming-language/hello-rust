use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let mut input = String::new();
    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Please type a number!");

    let mut i = 1;
    while i <= input {
        println!("{}", i);
        i += 1;
    }
}
