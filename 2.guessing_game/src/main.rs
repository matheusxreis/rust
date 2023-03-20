extern crate rand;

use std::cmp::Ordering;
use std::io;
use rand::Rng;

struct Guess {
    value:u32,
}

impl Guess {
    pub fn new(value:u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must be betwenn 1 and 100, you guessed: {}", value)
        }
        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

       
        let mut guess = String::new(); 
                                      
        io::stdin()
            .read_line(&mut guess) 
            .expect("Failed to read guess.");

        
        let guess = Guess::new(guess.trim().parse().unwrap());
        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue // -> continue follow the next interaction of the loop
        // };

        println!("You guessed: {}", guess.value);

   
        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

