use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    panic_with_result_expect();
}

fn unrecoverable_error() {
    loop {
        let mut option = String::new();

        println!("You want a panic manual or caused by a error in code?");
        println!("1. manual");
        println!("2. caused by the error");

        match io::stdin().read_line(&mut option) {
            Ok(opt) => (),
            Err(_) => continue,
        };

        // there is unrecoverable errors
        // they are errors that really are bugs
        // in this case, the rust go in panic

        if option.trim().eq("1") {
            //we can it manually with the macro panic!...
            panic!("Entering in panic....");
        } else if option.trim().eq("2") {
            //or in some cases, when the rust realizes that is
            //a bug, they do it by itself
            let vec = vec![1, 2, 3];
            let item = &vec[20];
        }
    }
}

fn recoverables_error() {
    // they are errors that are returned with Result enum,
    // a enum with 2 variants: Ok(T) and Err(E)
    // this enum is in standard library, so dont need be imported
    // some methods of standard library returns one Result enum
    // showing us that the error is not necessary a reason to panic

    // if i try open a unexisted file, it will return a Result with a error
    // and i can fix it by create a file
    // so there is no need to panic
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // match guard
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
           match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!(
                    "Tried create a file and there was a problem: {:?}",
                    e
                );
            },
           }
        },
        Err(error) => { 
            panic!("Problem to open file: {:?}", error);
        }
    };
}

fn panic_with_result(){

    // the Result enum has a method
    // unwrap that returns the value of Ok
    // if all is right, but if it is a Err
    // panic

    let f = File::open("notexistence.txt").unwrap();
}
fn panic_with_result_expect(){

    // the Result enum has a method
    // expect that returns the value of Ok
    // if all is right, but if it is a Err
    // panic, but with a personalized message

    let f = File::open("notexistence.txt").expect("WWWWWWWOOOOOOOOOW PANIC!!!!!");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    // o operator '?' only can be used in a function that returns a Result
    // case the file dont exist or there is no content inside it
    // a Result.Error will be returned from the whole function
    // otherwise the value in the file will go to mutable variable 's'
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}