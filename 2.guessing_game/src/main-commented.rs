// adving rust that you're using a external crate
extern crate rand;

// importing std::io library to input/output functions
use std::cmp::Ordering;
use std::io;
// Rng is a trait that define the methods of the number generator
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // let foo = 5 -> immutable
        // let mut bar = 5 -> mutable
        let mut guess = String::new(); // -> new String instance, a type of standard library that is growable, UTF-8 encoded bit of text
                                       // ::new is like a static method of string, a function associate to this type
                                       // io::stdion() returns a new instance of Stdin
                                       // the function read_line receives a mut string as paramater and returns a io::Result, which is a enum, that can be Ok or Err
        io::stdin()
            .read_line(&mut guess) // '&' represents a reference, to our code can access our variable in multiple parts without create a copy in memory
            // the io::Result instance has method called expect
            // case io::Result be a Err, expect stop the programa and display a message
            // case it be a Ok, expect returns the value
            // in this case the number of bytes of data that user inserted
            .expect("Failed to read guess.");

        // you can reasign the type of variable, reasign they in this way
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue // -> continue follow the next interaction of the loop
        };

        println!("You guessed: {}", guess);

        // the cmp method can be called in any variable that is comparable
        // cmp returns a Ordering enum, which is a enumerable pre-defined values
        // this can be Less, Greater or Equal
        // match verify the result and execute the line which is valid

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

// this project uses a CRATE
// a crate is a rust code package
// this project is a binary crate, that is executable
// rand, for example, is a library crate, which contains code that is to intended
// be used in other programs and can't be executed on its own
// the open source crate is in Crates.io (https://crates.io)
