// Associated Type === fancy words for Static Method
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut guess: String = String::new();
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    println!("Guess a number between 1 and 100!  Press C to Cheat and see the answer :)");
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess = if guess.trim().to_ascii_lowercase() == "c" {
        println!("The secret number is {}, try now", secret_number);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        return guess
            .trim()
            .parse::<u32>()
            .expect("You did not enter a number");
    } else {
        let guess = guess
            .trim()
            .parse()
            .expect("You did not enter a Number or C");
    };
    
    println!("You guessed {:?}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{} is low", guess),
        Ordering::Greater => println!("{} is high", guess),
        Ordering::Equal => println!("{} is correct!", guess),
    }
}
