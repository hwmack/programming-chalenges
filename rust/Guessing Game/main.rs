extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

// A small guess the number game
fn main() {
    println!("Guess the number!\n");

    loop {
        // Start playing the game
        play();

        let mut replay = String::new();
        println!("Would you like to replay (y/n)?");

        // Read line from stdin
        io::stdin().read_line(&mut replay).expect("Failed to read input");
        match replay.trim().as_ref() {
            "y" => continue,
            _ => { // Exit the game if the input isn't recognized
                println!("Goodbye!");
                return;
            }
        };
    }

}

fn play() {
    let mut try = 0;

    // Generate the random number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    while try < 5 {
        println!("Input your guess: ");
        let mut guess = String::new();

        // Read the users input from stdin
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a number!");
                continue
            }
        };

        // Check if the number is smaller or larger
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                return;
            }
        }

        try += 1;
    }

    println!("Too many guesses!\nThe number was: {}", secret_number);
}
