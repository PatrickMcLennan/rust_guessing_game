// Associated Type === fancy words for Static Method
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn error_message(&guess: &u32, &answer: &u32) {
    println!("You guessed: {}, and the correct answer is {}.  Please play again!",  guess, answer)
}

fn main() {
    let mut guess = String::new();
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
        
        guess
            .trim()
            .parse::<u32>()
            .expect("You did not enter a number")
    } else {
        guess
            .trim()
            .parse::<u32>()
            .expect("You did not enter a Number or C")
    };

    match guess.cmp(&secret_number) {
        Ordering::Less => error_message(&guess, &secret_number),
        Ordering::Greater => error_message(&guess, &secret_number),
        Ordering::Equal => println!("{} is correct!", guess),
    }
}
