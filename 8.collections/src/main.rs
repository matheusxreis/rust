mod hashmaps;
mod strings;
mod vectors;

fn main() {
    println!("Collection are mutable lists, saved in heap-memory.");
    println!("They are: vectors, strings and hash-maps.");

    //vectors::run();
    hashmaps::run();
}
