use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("GUess the number");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);    
    
    println!("the secret number is {secret_number}");

    println!("Plaese input your guess");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to real line");
    
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number");
    
    println!("you guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("to Small"),
        Ordering::Greater => println!("To big" ),
        Ordering::Equal => println!("You wind" )
    }
}
