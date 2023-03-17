use std::io;


const WHAT_ARE_CONST:&str = "const are ALWAYS immutables, they can be declared in any scope and never defined in execution time";

fn main() {

    let option_a = String::from("1");
    let option_b = String::from("2");

    loop {
        let mut option:String = String::new();

        println!("Do you want to know about 'const and let' or 'shadowing'?");
        println!("1. const and let");
        println!("2. shadowing");

        io::stdin()
        .read_line(&mut option)
        .expect("Something was wrong!");

            if option.trim().eq(&option_a) {
                println!("{}1", option);
                const_and_let();
                break;
            };
            if option.trim().eq(&option_b){
                println!("{}2", option);
                shadowing();
                break;
            };

       }
        

    }
   
fn const_and_let(){
    let mut x = WHAT_ARE_CONST;
    println!("{}.", x);
    x = "otherwise let variables, than can be mutables and defined in execution time.";
    println!("{}", x)
}

fn shadowing() {
    println!("shadowing is when you recreate the variable with same name, it means you shadow the variable");
    println!("despite you recreate the variable, it is not mut, because it is not reassigned, it is reCREATED and keeps immutable if it is the case");
    println!("it can be used to change the type of the variable.");
    let mut phrase_size = String::new();

    println!("input a string.");

     io::stdin()
    .read_line(&mut phrase_size);

    println!("what you typed {}", phrase_size);
    let phrase_size = phrase_size.trim().len();
    println!("the same variable now is not holding the phrase but your size: {}", phrase_size);
}