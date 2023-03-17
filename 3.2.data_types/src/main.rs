
const STATIC_TYPING:&str = "variables types are known at compile time";
const INFER_TYPE:&str = "the type can be infering by the compiler.";

fn main() {
    println!("Rust is a statically-typed language, thats means: {} and {}", STATIC_TYPING, INFER_TYPE);
    scalar_types();
    compound_types();
}


fn scalar_types(){
    println!("Rust has four primary scalar types: integers, floating-point numbers, booleans and characteres");

    let this_is_an_unsigned_int:u64 = 12;
    let this_is_a_signed_int:i64 = -12;
    let this_is_a_floating:f64 = 12.22;
    let this_is_a_boolean:bool = false;
    let this_is_a_character = 'c';

    println!("this is an unsigned int: {}, this is a signed int: {}, this is a floating-pointer number: {}, this is a boolean: {} and this is a character: {}",
    this_is_an_unsigned_int,
    this_is_a_signed_int,
    this_is_a_floating,
    this_is_a_boolean,
    this_is_a_character);
}

fn compound_types(){
  tuples();
  arrays();
}


fn tuples(){
    println!("Rust has two compound types, tuples and arrays.");

    let tup:(i32, f64, u8) = (-90, 5.8, 1);
    println!("My typle is: (-90, 5.8, 1)");
    println!("That's means: it has different types. I can access it by index or desestrucing");

    let (x, y, z) = tup;
    
    println!("let (x, y, z) = tup, x={x}, y={y}, z={z}");
    
    let first_index = tup.0;
    println!("....or, tup.0={first_index}");
}

fn arrays(){
    println!("Arrays are data types what has just one type and a pre-defined size. There is vectors, too, which allow you increase or decrease.");
    println!("If you want your data allocated in stack instead of heap, you need using an array or vector.");

    let arr = ["One", "Two", "Three", "Four", "Five", "Six", "Seven"];
    let arr2:[i32;5]=[1, 2, 3, 4, 5];

    println!("Creating a array:   let arr = ['One', 'Two', 'Three', 'Four', 'Five', 'Six', 'Seven'];");
    println!("Typing and defining: let arr:[i32;5]=[1, 2, 3, 4, 5]");
    println!("Accessing index of arr: arr[0].");


}