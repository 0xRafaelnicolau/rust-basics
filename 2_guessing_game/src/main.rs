use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("GUESS THE NUMBER!");

    loop {

        // this means we are creating a mutable variable
        // and assigning a new String to it.
        let mut guess = String::new();

        println!("Guess a number from 1 to 100!");

        // reference to the place in memory where
        // the string guess is stored, in order to
        // change the content of the reference the 
        // keyword "mut" must be used.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // the name guess is re-utilized because of something
        // called shadowing
        // when the ":" is used in variable declaration it is 
        // because we're annotating thee variable type.
        // match keyword is used to catch the Result of parse.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // first arm
            // the underscore means we are catching all types of errors.
            // the continue keyword makes it go to the next loop iteration.
            Err(_) => continue, // second arm 
        };

        // the expression 1..=100 is only possible due to
        // the rand::Rng imported.
        // generates a random number between 1 to 100.
        let secret_number = rand::thread_rng().gen_range(1..=100);
    
        // using the match keyword to catch the Result of cmp.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // first arm
            Ordering::Greater => println!("Too big!"), // second arm
            Ordering::Equal => { // third arm
                println!("You win!");
                break;
            }
        }

        println!("The secret number was: {secret_number}!");
    }
}
