pub fn run() {
    println!("Strings are UTF8 characters. &str is a slice of String");

    // ways to make a string
    let mut s = String::new();
    let mut s2 = String::from("a");
    let mut s3 = "string".to_string();

    // way to add a new string in front of another
    s.push_str("I'm");
    // s.push("a") -> push only a character

    println!("{}", format!["{} {} {}", s, s2, s3]);
    indexing();
}

fn indexing() {
    println!("A string can not be indexed because some characters need more than 1 byte of space, that's means: if you request a index in a string which contains a characters that ocuppies more than a byte, the response will be half of this characters and it will turns up a different char.");

    let s = "Здравствуйте"; // this characters ocuppies more than 1 byte of space
    let s2 = "abcdefgh"; // these too

    // s.char() // method char returns os characters as a vector

    for c in s.chars() {
        println!("{}", c);
    }
    for c in s2.chars() {
        println!("{}", c);
    }

    // s.bytes() // method bytes returns the bytes that compose the str

    for c in s.bytes() {
        println!("{}", c);
    }
    println!("=====");
    for c in s2.bytes() {
        println!("{}", c);
    }
}
