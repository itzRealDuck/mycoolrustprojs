use std::io;
extern crate rand;
use rand::Rng; 


fn main() {
    let mut rng = rand::thread_rng();

    let correct = rng.gen_range(1..100);

    let mut guess = String::new();
    let mut tries: u32 = 0;
    let mut guess2: u32 = 0;

    while guess2 != correct {
        println!("Number Guessing Game");
        println!("********************");
        println!("Enter a Random Number");
        io::stdin().read_line(&mut guess).unwrap();
        guess2 = guess.trim().parse().unwrap();
        tries += 1;

        if guess2 > correct {
            println!("Wrong Answer it is higher than the correct number");
            guess = String::new();
        } else if guess2 < correct {
            println!("Wrong the answer is wrong the guess is lower than the correct");
            guess = String::new();
        } else {
            println!("You are correct!");

            break;
        }
    }



println!("you had this many tries {} ", tries);

}
