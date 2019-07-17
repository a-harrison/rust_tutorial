use std::io;
use std::cmp::Ordering;
//use rand::Rng;

fn main() {
    println!("Guess the number!");

    // let secret_number = rand::thread_rng().gen_range(1, 101);
    let secret_number = 50;

    // println!("The secret number is: {} ", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // This block does not check that a user guesses between 1 and 100.
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // Prints message if value can't be stored within an i32
                println!("There was an issue parsing that value.");
                continue
            }
        };

        // Validates input and prints message for values out of range
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn between_zero_and_100() {
        Guess::new(50);
    }
}

pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
