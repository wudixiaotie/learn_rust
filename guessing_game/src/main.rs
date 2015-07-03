extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!\nPlease input your guess:");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is:{}", secret_number);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    println!("Your guessed: {}", guess);
}
