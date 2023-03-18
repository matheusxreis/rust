use ansi_term::Colour;

// ownership rules 
// 1. each value has a owner;
// 2. only can be a unique owner;
// 3. when owner get out of the scope, the value is cleaned (another languages has garbage collector or you make manually);
// 4. the ownership can be moved to another owner;

fn main() {
    ownership();
}


fn copy_types(){

    let a = 15;
    let b = a;

    println!("With copy types, we get deepy copies, that is: we create a new pointer and value.");
    println!("If we say: \n'let a= 15;'\n'let b=a;'\nso a is equal={a} and b, is equal {b}.");

    println!("DEEPY COPY");
    println!(" ---stack-----");
    println!("|  _      __  |");
    println!("| |a| -> |15| |");
    println!("| |b| -> |15| |");
    println!("|             |");
    println!(" -------------");


    let a = 15;
    let b = &a;
    println!("If we want to create a shallow copy with any copy types, we need copy the reference using '&'.");
    println!("'let a= 15;'\n'let b=a;'\nso a is equal={a} and b, is equal {b}");


    println!("SHALLOW COPY");
    println!(" ---stack-----");
    println!("|  _      __  |");
    println!("| |a| -> |15| |");
    println!("|  ^          |");
    println!("|  |          |");
    println!("| |b|         |");
    println!("|             |");
    println!(" -------------");
}

fn ownership(){

    println!("When we are talking about heap-memory-values, is very costly to make copies, so it dont happens in RUST.");
    println!("Here go the rust 'ownership' concept. All heap values has ONLY an owner, its owner can change and the value exists in heap memory just during the scope of its owner.");

    let a = String::from("Matheus");
    let b = a;


    println!("OWNERSHIP");
    println!(" ---stack-----  -----heap--------");
    println!("|  _      __  |     ____________|");
    println!("| |a| --------| -->|S('Matheus')| {} -> 'a' became {}",Colour::Red.paint("✗"), Colour::Red.paint("INVALID")); 
    println!("| |b| --------| -->|S('Matheus')| {} -> 'b' is {} of this heap value", Colour::Green.paint("✔"), Colour::Green.paint("THE NEW OWNER"));
    println!("|             |                 |");
    println!(" ------------   ----------------");

}