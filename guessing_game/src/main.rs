use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut range_start = 1;
    let mut range_end = 100;
    let secret_number = rand::thread_rng().gen_range(range_start..=range_end);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Guess the number in [{}, {}]!", range_start, range_end);
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Shadow previous value of guess (reuse name, but for different type)
        // Using a match expression instead of expect is a way to handle the
        // error, rather than just crashing on it
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ is a catchall value (matches all Err
                                // values, regardless of what information they
                                // contain)
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                range_start = guess + 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                range_end = guess - 1;
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
