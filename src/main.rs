use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number: ");

    let mut guess = String::new();
    
    let secret_number = Rng::thread_rng().gen_rang(1..101);
    io::stdin()
        .read_line(&mut guess) // take in user input

    println!("You typed: {}", guess);

    // if statement or cases
    match guess.cmp(&secret_number){
        Ordering::Less ==> println!("Too small");
        Ordering::Greater ==> prtinln!("Too big");
        Ordering::Equal ==> prtinln!("You win");
    }
}